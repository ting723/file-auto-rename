#!/bin/bash

# 检查脚本是否以 root 权限运行
if [ "$(id -u)" != "0" ]; then
    echo "此脚本需要以 root 权限运行。请使用 'sudo'。"
    exit 1
fi

cargo clean
cargo build --release

# 从 Cargo.toml 文件中提取应用名称
APP_NAME=$(grep "^name = " Cargo.toml | cut -d'"' -f2)

# 检查是否成功获取应用名称
if [ -z "$APP_NAME" ]; then
    echo "错误：无法从 Cargo.toml 中读取应用名称。"
    exit 1
fi

# 构建二进制文件路径
BINARY_PATH="target/release/$APP_NAME"

# 检查二进制文件是否存在
if [ ! -f "$BINARY_PATH" ]; then
    echo "错误：无法找到二进制文件 '$BINARY_PATH'"
    exit 1
fi

# 检查 /usr/local/bin 目录是否存在，如果不存在则创建
if [ ! -d "/usr/local/bin" ]; then
    mkdir -p /usr/local/bin
fi

# 检查目标目录中是否有同名的可执行文件
if [ -f "/usr/local/bin/$APP_NAME" ]; then
    read -p "检测到已有文件 '/usr/local/bin/$APP_NAME'。是否覆盖？[y/n]: " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# 复制二进制文件到 /usr/local/bin
cp "$BINARY_PATH" "/usr/local/bin/$APP_NAME"

# 设置执行权限
chmod +x "/usr/local/bin/$APP_NAME"

echo "安装完成！'$APP_NAME' 已经复制到 '/usr/local/bin/'。"
