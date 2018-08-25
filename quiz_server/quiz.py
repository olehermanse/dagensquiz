import json
from collections import OrderedDict

from quiz_server.randomish import shuffle, randint, pick, hash

quiz_count = 0
quizzes = {}
quiz_data = {}


def read_json(path):
    with open(path, "r", encoding='utf-8') as f:
        d = json.loads(f.read(), object_pairs_hook=OrderedDict)
    return d


def read_questions(path):
    with open(path, "r", encoding='utf-8') as f:
        lines = f.readlines()
    questions = []
    answers = []
    for line in lines:
        q, a = line.split("    - ")
        questions.append(q.strip())
        answers.append(a.strip())
    return questions, answers


def fill_category(category, lang):
    if "questions" in category:
        filename = category["questions"]
        assert type(filename) is str
        questions, answers = read_questions("2018/{}/{}".format(
            lang, filename))
        category["questions"] = []
        for q, a in zip(questions, answers):
            obj = {}
            obj["question"] = q
            obj["answer"] = a
            category["questions"].append(obj)

    if "subcategories" in category:
        subcategories = category["subcategories"]
        for sub in subcategories:
            fill_category(sub, lang)


def init(lang):
    structure = read_json("2018/{}/data.json".format(lang))
    categories = structure["categories"]
    for cat in categories:
        fill_category(cat, lang)
    global quiz_data
    quiz_data = structure


def pick_question(category, n, seed):
    questions = category["questions"]
    if len(questions) == 0:
        return None
    return pick(questions, seed, n)


def pretty(data):
    return json.dumps(data, indent=2)


def gen_quiz(seed, length=10):
    state = (length + seed) // 2
    questions = []
    answers = []
    categories = quiz_data["categories"]
    subcategories = []
    for category in categories:
        if "subcategories" in category:
            subcategories.extend(category["subcategories"])

    question_categories = categories + subcategories
    while len(question_categories) < length:
        r = randint(0, len(question_categories), [seed, state])
        category = question_categories[r]
        question_categories.append(category)
        state += 1
    for category in question_categories:
        if "questions" not in category:
            continue
        q, a = None, None
        obj = None
        while q is None or a is None or q in questions or a in answers:
            obj = pick_question(category, seed, state)
            if not obj:
                continue
            q, a = obj["question"], obj["answer"]
            state += 1
        questions.append(q)
        answers.append(a)
    qa = zip(questions, answers)
    qa = shuffle(qa, seed)
    qa = qa[0:length]
    questions, answers = zip(*qa)
    return [questions, answers]


def get_quiz(quiz_id):
    seed = hash(quiz_id)
    global quizzes
    global quiz_count
    global quiz_data
    if seed in quizzes:
        return quizzes[seed]

    r = gen_quiz(seed)
    if quiz_count >= 100:
        quizzes = {}
        quiz_count = 0
    quizzes[seed] = r
    quiz_count += 1
    return r
