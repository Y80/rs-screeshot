#!/bin/bash

# 确保脚本在任何错误时立即停止
set -e

# 重置本地仓库并拉取最新代码
git reset --hard
git pull

# 定义可执行文件路径
exec_path="$HOME/codes/rs-screenshot/target/release/screenshot"

# 如果进程正在运行，则杀死它
if pgrep -f "$exec_path" > /dev/null; then
  kill $(pgrep -f "$exec_path")
fi

# 重新启动进程，并将输出重定向到指定文件
nohup "$exec_path" > "$HOME/codes/rs-screenshot/screenshot.log" 2>&1 &
