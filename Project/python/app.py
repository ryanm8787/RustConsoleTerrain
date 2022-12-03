import os

from flask import (
    Blueprint, flash, g, Flask, redirect, render_template, request, session, url_for
)

app = Flask(__name__)


# a simple page that says hello
@app.route('/')
def test_page():
    return render_template('base.html')


if __name__ == "__main__":
    port = int(os.environ.get('PORT', 5000))
    app.run(debug=True, host='0.0.0.0', port=port)