use axum::{body::Bytes, extract::Multipart};
use image::io::Reader as ImageReader;
use std::io::Cursor;

pub async fn red_pixels(mut multipart: Multipart) -> String {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == String::from("image") {
            return process_image_data(&data);
        }
    }

    String::from("something happened")
}

fn process_image_data(data: &Bytes) -> String {
    let img = ImageReader::new(Cursor::new(data));

    let opened_file = img.with_guessed_format().expect("open file error");
    let decoded_img = opened_file.decode().expect("decoded image error");

    let pixels = decoded_img
        .as_rgb8()
        .expect("error converting to rgb8")
        .pixels();

    let magic_red_pixels = pixels.fold(0, |sum, pixel| {
        if pixel[0] as u16 > pixel[1] as u16 + pixel[2] as u16 {
            return sum + 1;
        }

        sum
    });

    format!("{}", magic_red_pixels)
}
