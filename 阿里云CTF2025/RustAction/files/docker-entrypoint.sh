#!/bin/bash

echo $FLAG > /flag
unset FLAG

chown ctf:ctf /flag
chmod 000 /flag

su -p ctf -c "./action conf/config.toml"
