import os
import json
import utilities
import requests

from flask import (
    Blueprint, flash, g, Flask, redirect, render_template, request, session, url_for
)

app = Flask(__name__)

my_text = {"data": "", "width": 0, "height": 0}

@app.route('/')
def test_page():
    # TODO: handle exception.
    requests.post('http://172.17.0.2:7878', json={"tempt":"r"})
    return render_template('base.html')


@app.route('/data', methods=('GET', 'POST'))
def current_data():
    global my_text
    if request.method == 'POST':
        my_text = json.dumps(request.get_json())

    return my_text


if __name__ == "__main__":
    port = int(os.environ.get('PORT', 5000))
    app.run(debug=True, host='0.0.0.0', port=port)