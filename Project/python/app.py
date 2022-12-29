import os
import json

from flask import (
    Blueprint, flash, g, Flask, redirect, render_template, request, session, url_for
)

app = Flask(__name__)


@app.route('/')
def test_page():
    return render_template('base.html')


@app.route('/data', methods=('GET', 'POST'))
def current_data():
    my_text = "<p>Nice test.</p>"
    print(request)
    if request.method == 'POST':
        print(request)
        my_text = "<p>" + json.dumps(request.get_json()) + "</p>"

    return my_text


if __name__ == "__main__":
    port = int(os.environ.get('PORT', 5000))
    app.run(debug=True, host='0.0.0.0', port=port)