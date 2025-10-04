#!/bin/sh

chmod 600 /entrypoint.sh

if [ ${ICQ_FLAG} ];then
    echo -n ${ICQ_FLAG} > /flag
    chown root:root /flag
    chmod 400 /flag
    echo [+] ICQ_FLAG OK
    unset ICQ_FLAG
else
    echo [!] no ICQ_FLAG
fi

(cd /sandbox && (./sandbox&))

echo [*] Waiting for sandbox to start
sleep 1

echo [*] Starting app
su app -c '
(cd /app && (python3 app.py&))
'

echo [*] tailing /dev/null
tail -f /dev/null