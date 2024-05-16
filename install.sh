#!/bin/bash

# 클론할 리포지토리 URL
REPO_URL="https://github.com/steelPiber/rsjpg.git"
# 클론할 디렉토리 이름
REPO_DIR="rsjpg"

function install_rsjpg {
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
  sudo cp target/release/rsjpg /usr/local/bin/

  echo "Installation complete."
}

function uninstall_rsjpg {
  # 바이너리를 /usr/local/bin에서 삭제
  echo "Removing the binary..."
  sudo rm /usr/local/bin/rsjpg

  # 프로젝트 디렉토리 삭제
  echo "Removing the project directory..."
  rm -rf ~/"$REPO_DIR"

  echo "Uninstallation complete."
}

if [ "$1" == "install" ]; then
  install_rsjpg
elif [ "$1" == "uninstall" ]; then
  uninstall_rsjpg
else
  echo "Usage: $0 {install|uninstall}"
fi
