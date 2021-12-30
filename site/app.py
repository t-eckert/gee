from flask import Flask, render_template
from markdown import markdown

import httpx

app = Flask(__name__)


@app.route("/")
def home():
    return render_template("home.html")


@app.route("/docs/<name>")
def docs(name):
    response = httpx.get(
        f"https://raw.githubusercontent.com/t-eckert/gee/main/docs/{name}.md"
    )
    if not response.is_success:
        return render_template("404.html")

    doc = markdown(response.text)

    return render_template("docs.html", doc=doc)
