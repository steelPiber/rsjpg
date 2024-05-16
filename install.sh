#!/bin/bash

# 클론할 리포지토리 URL
REPO_URL="https://github.com/steelPiber/rsjpg.git"

# 클론할 디렉토리 이름
REPO_DIR="rsjpg"

# 프로젝트 클론
echo "Cloning the project..."
git clone "$REPO_URL"

# 디렉토리로 이동
cd "$REPO_DIR" || { echo "Failed to change directory to $REPO_DIR"; exit 1; }

# 프로젝트 빌드 및 설치
echo "Building the project..."
cargo build --release

# 바이너리를 /usr/local/bin으로 복사
echo "Installing the binary..."
cp target/release/rsjpg /usr/local/bin/

echo "Installation complete."

rm -rf "$REPO_URL"
