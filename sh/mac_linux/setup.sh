#!/bin/bash

# 获取用户主目录路径
HOME_DIR="$HOME"

# 创建 Tiks 文件夹
APP_DIR="$HOME_DIR/.Tiks"
mkdir -p "$APP_DIR"


# create your user_pd
USER_FILE="$APP_DIR/user"
touch "$USER_FILE"

APP_DIR_BIN="$HOME_DIR/.Tiks/bin"
mkdir -p "$APP_DIR_BIN"