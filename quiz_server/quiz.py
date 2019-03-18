import os
import json
from collections import OrderedDict

from quiz_server.randomish import shuffle, randint, pick, hash


def read_json(path):
    with open(path, "r", encoding='utf-8') as f:
        return json.loads(f.read(), object_pairs_hook=OrderedDict)


def write_json(data, path):
    with open(path, "w", encoding='utf-8') as f:
        f.write(pretty(data))


def read_questions(path):
    with open(path, "r", encoding='utf-8') as f:
        lines = f.readlines()
    questions = []
    for line in lines:
        q, a = line.split("  - ")
        questions.append([q.strip(), a.strip()])
    return questions


def init():
    data = OrderedDict()
    languages = ["en", "no"]
    for l in languages:
        data[l] = OrderedDict()
        categories = data[l]["categories"] = OrderedDict()
        files = os.listdir("2018/{}/".format(l))
        files.sort()
        question_files = list(filter(lambda x: x.endswith(".txt"), files))
        for file in question_files:
            categories[file] = read_questions("2018/{}/{}".format(l, file))

    write_json(data, "output.json")

    global quiz_data
    quiz_data = data
    assert quiz_data


def pick_question(questions, n, seed):
    if len(questions) == 0:
        return None
    return pick(questions, seed, n)


def pretty(data):
    return json.dumps(data, indent=2)


def gen_quiz(seed, language, length=10):
    questions = []
    answers = []
    categories = quiz_data[language]["categories"]

    categories = [x for x in categories]
    categories = list(categories)
    categories = shuffle(categories, seed)
    while len(categories) < length:
        categories += categories
    categories = categories[0:length]
    categories = categories[0:length // 2] + shuffle(categories[length // 2:],
                                                     seed)

    assert len(categories) == length

    state = length + seed + 1
    for category in categories:
        q, a = None, None
        category = quiz_data[language]["categories"][category]
        for _ in range(5):
            obj = pick_question(category, seed, state)
            state += 1
            if not obj:
                continue
            q, a = obj
            if q in questions or a in answers:
                continue
            questions.append(q)
            answers.append(a)
            break
    qa = zip(questions, answers)
    qa = shuffle(qa, state)
    qa = qa[0:length]
    questions, answers = zip(*qa)
    return [questions, answers]


def get_quiz(quiz_id, language):
    seed = hash(quiz_id)
    return gen_quiz(seed, language)
