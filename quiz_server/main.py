import argparse
import random


from flask import Flask, render_template, redirect, abort
from flask_compress import Compress

from quiz_server.quiz import get_quiz, init

app = Flask(__name__, static_url_path='', static_folder='static')
Compress(app)


# Web server:
@app.route('/')
def root():
    timestamp = "2018-07-01"
    return redirect("/{}".format(timestamp))


@app.route('/<string:quiz_id>', methods=["GET"])
def editor(quiz_id):
    if "favicon.ico" == quiz_id:
        abort(404)
    questions, answers = get_quiz(quiz_id)
    return render_template(
        'index.html', date=quiz_id, questions=questions, answers=answers)


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
    init()
    start_server(args.ip, args.port)


if __name__ == "__main__":
    raise AssertionError
