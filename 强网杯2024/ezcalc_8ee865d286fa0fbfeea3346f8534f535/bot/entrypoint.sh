#!/bin/sh

su bot -c '
(cd /bot && (yarn start&))
'

tail -f /dev/null