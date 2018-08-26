import os
import argparse
import random

from datetime import date

from flask import Flask, render_template, redirect, abort
from flask_compress import Compress

from quiz_server.quiz import get_quiz, init

app = Flask(__name__, static_url_path='', static_folder='static')
Compress(app)

NORWEGIAN = {
    "title": "Dagens Quiz",
    "url": "https://dagensquiz.no/",
    "more": "Mer quiz...",
    "solution": "Dagens Fasit",
    "id": "no"
}

ENGLISH = {
    "title": "Daily Quiz",
    "url": "https://dailyquiz.app/",
    "more": "More quiz...",
    "solution": "Solution",
    "id": "en"
}

LANG = NORWEGIAN
print("ENV:")
print(str(os.environ))

if "ENGLISH_QUIZ" in os.environ and os.environ["ENGLISH_QUIZ"] == "1":
    LANG = ENGLISH


# Web server:
@app.route('/')
def root():
    today = date.today()
    timestamp = today.strftime('%Y-%m-%d')
    return redirect("/{}".format(timestamp))


@app.route('/<string:quiz_id>', methods=["GET"])
def editor(quiz_id):
    global LANG
    if "favicon.ico" == quiz_id:
        abort(404)
    if quiz_id == "random":
        return redirect("/{}".format(random.randint(0, 1000)))
    questions, answers = get_quiz(quiz_id)
    subtitle = quiz_id
    if "-" not in subtitle:
        subtitle = "#" + subtitle
    return render_template(
        'index.html',
        text_title=LANG["title"],
        text_solution=LANG["solution"],
        text_more=LANG["more"],
        url=LANG["url"],
        date=subtitle,
        questions=questions,
        answers=answers)


def get_args():
    argparser = argparse.ArgumentParser(
        description='Nowcode webserver/backend')
    argparser.add_argument(
        '--ip', '-i', help='IP', type=str, default="0.0.0.0")
    argparser.add_argument(
        '--port', '-p', help='port number', type=int, default=5000)
    argparser.add_argument(
        '--release', '-r', help='Release mode', action="store_true")
    args = argparser.parse_args()
    return args


def start_server(ip, port):
    app.run(ip, port=port)


def main():
    args = get_args()
    if args.release:
        pass
    else:
        app.config['SERVER_NAME'] = "127.0.0.1:5000"
        app.config['DEBUG'] = True
        app.config['TEMPLATES_AUTO_RELOAD'] = True
    global LANG
    init(lang=LANG["id"])
    start_server(args.ip, args.port)


if __name__ == "__main__":
    raise AssertionError
