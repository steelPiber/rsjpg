# rsjpg
`rsjpg`는 여러 이미지를 하나의 이미지로 결합하는 Rust로 작성된 명령줄 도구입니다.
`rsjpg` is a command-line tool written in Rust that combines multiple images into one image.
## install  | 설치 

```sh
git clone https://github.com/steelPiber/rsjpg.git

cd rsjpg

cargo build --release

cd rsjpg/target/release
./rsjpg
```

```sh
cd rsjpg
./install.sh
```
```sh
wget -qO- https://github.com/steelPiber/rsjpg/raw/main/install.sh | bash
```
## help | 사용법
  * -n, --name <OUTPUT>: 출력 파일의 이름을 지정합니다.
  * -m, --mode <MODE>: 이미지를 가로(w) 또는 세로(h)로 결합합니다. 가능한 값: w, h.
  * -h, --help: 도움말 메시지를 출력합니다.
  * -V, --version: 버전 정보를 출력합니다.
