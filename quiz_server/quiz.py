import hashlib
import json
from collections import OrderedDict

from quiz_server.randomish import shuffle, randint

quiz_count = 0
quizzes = {}
quiz_data = {}

def read_json(path):
    with open(path, "r") as f:
        d = json.loads(f.read(), object_pairs_hook=OrderedDict)
    return d

def read_questions(path):
    with open(path, "r") as f:
        lines = f.readlines()
    questions = []
    answers = []
    for line in lines:
        q,a = line.split("    - ")
        questions.append(q.strip())
        answers.append(a.strip())
    print(questions)
    print(answers)
    return questions, answers

def fill_category(category):
    filename = category["questions"]
    assert type(filename) is str
    questions, answers = read_questions("2018/no/" + filename)
    category["questions"] = []
    for q,a in zip(questions, answers):
        obj = {}
        obj["question"] = q
        obj["answer"] = a
        category["questions"].append(obj)

    if "subcategories" in category:
        subcategories = category["subcategories"]
        for sub in subcategories:
            fill_category(sub)

def init():
    structure = read_json("2018/no/data.json")
    categories = structure["categories"]
    for cat in categories:
        fill_category(cat)
    global quiz_data
    quiz_data = structure

def pick_question(category, n, seed):
    categories = [category]
    if "subcategories" in category:
        categories.extend(category["subcategories"])
    ind = (n +  seed) % len(categories)
    new_cat = categories[ind]
    if new_cat != category:
        return pick_question(new_cat, n, seed)
    category = categories[ind]
    questions = category["questions"]
    if len(questions) == 0:
        return None
    ind = (2 * n + seed) % len(questions)
    return questions[ind]


def gen_quiz(seed):
    questions = []
    answers = []
    categories = quiz_data["categories"]
    n = 0
    while len(questions) < 10 and n < 100:
        cat_ind = randint(seed, n)
        cat_ind = cat_ind % len(categories)
        category = categories[cat_ind]
        obj = pick_question(category, n, seed)
        n += 1
        if obj is None:
            continue
        q,a = obj["question"], obj["answer"]
        if q in questions or a in answers:
            continue
        questions.append(q)
        answers.append(a)
    qa = zip(questions, answers)
    qa = shuffle(qa, seed)
    questions, answers = zip(*qa)
    return [questions, answers]

def get_quiz(quiz_id):
    seed = hash(quiz_id)
    global quizzes
    global quiz_count
    global quiz_data
    if seed in quizzes:
        print("Cache hit!")
        return quizzes[seed]
    print("Cache miss!")

    r = gen_quiz(seed)
    if quiz_count >= 100:
        quizzes = {}
        quiz_count = 0
    quizzes[seed] = r
    quiz_count += 1
    return r

def hash(string):
    bytes = string.encode('utf-8')
    hasher = hashlib.sha256()
    hasher.update(bytes)
    digest = hasher.digest()
    num = int.from_bytes(digest[0:4], byteorder='big', signed=False)
    return num
