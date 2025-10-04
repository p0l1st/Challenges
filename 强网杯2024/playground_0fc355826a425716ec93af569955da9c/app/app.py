import base64
from flask import Flask, render_template, request, jsonify
import struct
import socket
import io
import os

app = Flask(__name__)
key = bytes.fromhex(open('/sandbox/sandbox.key', 'r').read())
indexHTML = open('index.html', 'r').read()

def run_in_sandbox(prog):
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    s.connect(('localhost', 2077))
    chall = bytearray(s.recv(1024))
    for i in range(len(chall)):
       chall[i] ^= key[i % len(key)]
    
    prog = bytearray(prog)
    for i in range(len(prog)):
        prog[i] ^= key[i % len(key)]

    options = 52
    pkt = io.BytesIO()
    pkt.write(struct.pack('<II', len(prog) + len(chall), options))
    pkt.write(chall)
    pkt.write(prog)
    s.send(pkt.getvalue())

    output = io.BytesIO()
    while True:
        try:
            data = s.recv(1024)
            if not data:
                break
            output.write(data)
        except:
            break

    s.close()
    return output.getvalue()


@app.post('/api/run')
def run():
    code = request.json['code']
    dirname = os.urandom(24).hex()
    os.mkdir(f'/tmp/{dirname}')
    with open(f'/tmp/{dirname}/main.go', 'w') as f:
        f.write(code)
    ret = os.system(f'cd /tmp/{dirname}/ && go mod init playground && go build')
    
    if ret != 0 or not os.path.exists(f'/tmp/{dirname}/playground'):
        os.system(f'rm -rf /tmp/{dirname}')
        return jsonify({'status': 'error'})
    
    prog = open(f'/tmp/{dirname}/playground', 'rb').read()
    os.system(f'rm -rf /tmp/{dirname}')
    output = run_in_sandbox(prog)
    return jsonify({'status': 'success', 'data': base64.b64encode(output).decode()})

@app.get('/')
def index():
    headers = {
        'Content-Type': 'text/html',
    }
    return (indexHTML, 200, headers)


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)