use clap::{Arg, Command};

pub fn command_list() -> Command {
    Command::new("Image Combiner")
        .version("0.1")
        .author("pibre[pyh5523@gmail.com]")
        .about("Combines multiple images into a single image | 여러 이미지를 하나의 이미지로 결합")
        .arg(
            Arg::new("name") //명령어 인자 선언
                .short('n') //옵션 : -n 명령어 수행
                .long("name") //옵션 : --name 명령어 수행
                .value_name("OUTPUT") //<OUTPUT>  값 이름을 help로 보임
                .required(true) //인자 값을 가져옴
                .help("Specify the name of the output file | 입력된 문자열로 파일 이름 출력"),
        )
        .arg(
            Arg::new("mode")//명령어 인자 선언
                .short('m')//옵션 :-m
                .long("mode") //옵션 : --mode
                .value_name("MODE") // <MODE> 값 이름을 help로 보임
                .required(true) //인자 값을 가져옴
                .value_parser(["w","h"]) // -w -h 인자 값 파싱 함
                .help("Combine images horizontally (w) or vertically (h) | 이미지를 가로(w) 또는 세로(h) "),
         )
         .arg(
             Arg::new("images") //명령어 인자 선언
                 .required(true)//인자 값을 가져옴
                 .value_name("IMAGES") //  <IMAGES> 값 이름을 help로 보임
                 .num_args(1..) //n개의 개수를 인자 값을 가져옴
                .help("List of images to combine | 결합할 이미지 목록"),
         )
}
