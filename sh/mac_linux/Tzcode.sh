#!/bin/bash

HOME_DIR="$HOME"

TZCODE = "$HOME_DIR"/Tzcode
mkdir -p "$TZCODE"

bash ./setup.sh

INI_TZCODE = "$TZCODE"/env.ini
touch "$INI_TZCODE"

python3 ../../py/ini.py