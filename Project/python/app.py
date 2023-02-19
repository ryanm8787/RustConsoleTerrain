import os
import json
import utilities

from flask import (
    Blueprint, flash, g, Flask, redirect, render_template, request, session, url_for
)

app = Flask(__name__)

my_text = ""

@app.route('/')
def test_page():
    return render_template('base.html')


@app.route('/data', methods=('GET', 'POST'))
def current_data():
    global my_text
    if request.method == 'POST':
        my_text = json.dumps(request.get_json())
        # arr = utilities.parse_json_raw_to_arr(request.get_json())

    return my_text


if __name__ == "__main__":
    port = int(os.environ.get('PORT', 5000))
    app.run(debug=True, host='0.0.0.0', port=port)