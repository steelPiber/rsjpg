
#!/bin/bash
set -e

# 프로젝트 빌드
echo "Building the project..."
cargo build --release

# 바이너리 설치 디렉토리
INSTALL_DIR="/usr/local/bin"

# 바이너리 이름
BINARY_NAME="rsjpg"

# 빌드된 바이너리 경로
BINARY_PATH="target/release/$BINARY_NAME"

# 바이너리 설치
echo "Installing $BINARY_NAME to $INSTALL_DIR..."
sudo cp "$BINARY_PATH" "$INSTALL_DIR/"

# 설치 확인
if [ -x "$(command -v $BINARY_NAME)" ]; then
    echo "$BINARY_NAME installed successfully!"
else
    echo "Installation failed!"
    exit 1
fi
