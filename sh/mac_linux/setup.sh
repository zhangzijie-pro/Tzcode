#!/bin/bash

# 获取用户主目录路径
HOME_DIR="$HOME"

# 创建 Tiks 文件夹
COMMAND_DIR="$HOME_DIR/.Tiks"
mkdir -p "$COMMAND_DIR"

# create your user_pd
# write in tiks_command-> root
USER_FILE="$COMMAND_DIR/user.ini"
touch "$USER_FILE"

APP_DIR_BIN="$COMMAND_DIR/bin"
mkdir -p "$APP_DIR_BIN"