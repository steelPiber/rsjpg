use image::{DynamicImage, GenericImageView, RgbaImage};
use std::path::Path;

mod arg;
mod img;

fn main() {
    // 명령줄 인수 파싱
    let arg_command = arg::command_list().get_matches();

    // -w or -h
    let mode = arg_command.get_one::<String>("mode").unwrap();
    // 출력 파일 이름
    let output_path = arg_command.get_one::<String>("name").unwrap();

    // 입력된 경로들의 사진
    let input_paths: Vec<&str> = arg_command
        .get_many::<String>("images")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    // 사진들을 불러 객체화 생성
    let images = img::image_combination(input_paths);

    // Calculate the size of the combined image
    // 연산된 이미지 크기를 결합
    let (width, height) = match img::w_h_img(mode, &images) {
        Ok((w, h)) => (w, h),
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    // Create a new empty image with the combined size
    // 결합된 크기로 새 빈 이미지 생성
    let combined_image = RgbaImage::new(width, height);

    // Copy each image into the combined image
    // 각 이미지를 결합된 이미지로 복사
    let new_img = img::combined_img(combined_image, &images, mode);

    // Save the combined image as the specified output file
    // 결합된 이미지를 지정된 출력 파일로 저장
    new_img
        .save(output_path)
        .expect("Failed to save combined image");
}
