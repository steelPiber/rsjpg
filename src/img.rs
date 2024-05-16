use image::{DynamicImage, GenericImageView, RgbaImage};
use std::path::Path;

pub fn image_combination(arg_input: Vec<&str>) -> Vec<DynamicImage> {
    // 해당 경로의 이미지를 불러 객체화 생성
    arg_input
        .iter()
        .map(|&path| image::open(&Path::new(path)).expect("Failed to load image"))
        .collect()
}

// Calculate the size of the combined image
// 모드에 따른 가로, 세로 이미지 객체 크기 변화
pub fn w_h_img(modes: &str, img_vec: &[DynamicImage]) -> Result<(u32, u32), String> {
    match modes {
        "w" => {
            let (width, height) = img_vec
                .iter()
                .fold((0, 0), |(w, h), img| (w + img.width(), h.max(img.height())));
            Ok((width, height))
        }
        "h" => {
            let (width, height) = img_vec
                .iter()
                .fold((0, 0), |(w, h), img| (w.max(img.width()), h + img.height()));
            Ok((width, height))
        }
        _ => Err(format!(
            "잘못된 모드입니다: {}. 수평에 -w를 사용하거나 수직에 -h를 사용합니다.",
            modes
        )),
    }
}

// 결합된 크기로 이미지 반환
pub fn combined_img(
    mut empty_image: RgbaImage,
    img_vec: &[DynamicImage],
    modes: &str,
) -> RgbaImage {
    let mut x_offset = 0;
    let mut y_offset = 0;
    for img in img_vec {
        for (x, y, pixel) in img.pixels() {
            empty_image.put_pixel(x + x_offset, y + y_offset, pixel);
        }

        match modes {
            "w" => {
                x_offset += img.width();
            }
            "h" => {
                y_offset += img.height();
            }
            _ => {}
        }
    }
    empty_image
}
