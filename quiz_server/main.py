import os
import argparse
import random

from datetime import date

import flask
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


# Web server:
@app.route('/')
def root():
    today = date.today()
    timestamp = today.strftime('%Y-%m-%d')
    return redirect("/{}".format(timestamp))


@app.route('/<string:quiz_id>', methods=["GET"])
def editor(quiz_id):
    lang = NORWEGIAN
    if "dailyquiz" in flask.request.headers['Host']:
        lang = ENGLISH
    if "favicon.ico" == quiz_id:
        abort(404)
    if quiz_id == "random":
        return redirect("/{}".format(random.randint(0, 1000)))
    questions, answers = get_quiz(quiz_id, lang["id"])
    subtitle = quiz_id
    if "-" not in subtitle:
        subtitle = "#" + subtitle
    return render_template(
        'index.html',
        text_title=lang["title"],
        text_solution=lang["solution"],
        text_more=lang["more"],
        url=lang["url"],
        date=subtitle,
        questions=questions,
        answers=answers)


def get_args():
    argparser = argparse.ArgumentParser(description='Quiz webserver')
    argparser.add_argument(
        '--ip', '-i', help='IP', type=str, default="0.0.0.0")
    argparser.add_argument(
        '--port', '-p', help='port number', type=int, default=5000)
    args = argparser.parse_args()
    return args


def main():
    args = get_args()
    init()
    app.run(args.ip, port=args.port)
