#!/bin/sh

su app -c '
(cd /app && (./calc&))
'

tail -f /dev/null