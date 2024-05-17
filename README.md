# rsjpg
* `rsjpg`는 여러 이미지를 하나의 이미지로 결합하는 Rust로 작성된 명령줄 도구입니다.
* `rsjpg` is a command-line tool written in Rust that combines multiple images into one image.
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
### install wget
```sh
wget -qO- https://github.com/steelPiber/rsjpg/raw/main/install.sh | bash -s install
```
### uninstall wget
```sh
wget -qO- https://github.com/steelPiber/rsjpg/raw/main/install.sh | bash -s uninstall
```
## help | 사용법
  * -n, --name <OUTPUT>: 출력 파일의 이름을 지정합니다.
  * -m, --mode <MODE>: 이미지를 가로(w) 또는 세로(h)로 결합합니다. 가능한 값: w, h.
  * -h, --help: 도움말 메시지를 출력합니다.
  * -V, --version: 버전 정보를 출력합니다.


## help | How to use
   * -n, --name <OUTPUT>: Specifies the name of the output file.
   * -m, --mode <MODE>: Combine images horizontally (w) or vertically (h). Possible values: w, h.
   * -h, --help: Print help messages.
   * -V, --version: Print version information.

![화면-기록-2024-05-17-오후-3 02 58](https://github.com/steelPiber/rsjpg/assets/43775386/0f7f7306-4e20-496b-9060-5f1634c52c62)
