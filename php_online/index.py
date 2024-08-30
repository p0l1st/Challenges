from flask import Flask, request, session, redirect, url_for, render_template
import os
import secrets


app = Flask(__name__)
app.secret_key = secrets.token_hex(16)
working_id = []


@app.route('/', methods=['GET', 'POST'])
def index():
    if request.method == 'POST':
        id = request.form['id']
        if not id.isalnum() or len(id) != 8:
            return '无效的ID'
        session['id'] = id
        if not os.path.exists(f'/sandbox/{id}'):
            os.popen(f'mkdir /sandbox/{id} && chown www-data /sandbox/{id} && chmod a+w /sandbox/{id}').read()
        return redirect(url_for('sandbox'))
    return render_template('submit_id.html')


@app.route('/sandbox', methods=['GET', 'POST'])
def sandbox():
    if request.method == 'GET':
        if 'id' not in session:
            return redirect(url_for('index'))
        else:
            return render_template('submit_code.html')
    if request.method == 'POST':
        if 'id' not in session:
            return 'no id'
        user_id = session['id']
        if user_id in working_id:
            return 'task is still running'
        else:
            working_id.append(user_id)
            code = request.form.get('code')
            os.popen(f'cd /sandbox/{user_id} && rm *').read()
            os.popen(f'sudo -u www-data cp /app/init.py /sandbox/{user_id}/init.py && cd /sandbox/{user_id} && sudo -u www-data python3 init.py').read()
            os.popen(f'rm -rf /sandbox/{user_id}/phpcode').read()
            
            php_file = open(f'/sandbox/{user_id}/phpcode', 'w')
            php_file.write(code)
            php_file.close()

            result = os.popen(f'cd /sandbox/{user_id} && sudo -u nobody php phpcode').read()
            os.popen(f'cd /sandbox/{user_id} && rm *').read()
            working_id.remove(user_id)

            return result


if __name__ == '__main__':
    app.run(debug=False, host='0.0.0.0', port=80)
