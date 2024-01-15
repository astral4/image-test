use image::{
    codecs::avif::AvifEncoder, load_from_memory_with_format, ColorType, ImageEncoder, ImageFormat,
};
use std::io::sink;

fn main() {
    let data = include_bytes!("./test.png");

    let img = load_from_memory_with_format(data, ImageFormat::Png)
        .unwrap()
        .into_rgba8();

    drop(AvifEncoder::new(sink()).write_image(&img, img.width(), img.height(), ColorType::Rgba8));
}
