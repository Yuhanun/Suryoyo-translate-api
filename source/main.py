#!/usr/bin/python3

import flask

import backend

app = flask.Flask(__name__)

@app.route("/<term>")
def get_translated_term(term: str):
    return backend.get_translations_for(term)

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8000)