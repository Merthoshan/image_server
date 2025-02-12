use image::{DynamicImage, imageops};


pub fn grayscale_image(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}


pub fn invert_image(img: DynamicImage) -> DynamicImage {
    let mut rgba = img.to_rgba8();
    imageops::invert(&mut rgba);
    DynamicImage::ImageRgba8(rgba)
}


pub fn rotate90_image(img: DynamicImage) -> DynamicImage {
    DynamicImage::ImageRgba8(imageops::rotate90(&img.to_rgba8()))
}

pub fn blur_image(img: DynamicImage, sigma: f32) -> DynamicImage {
    DynamicImage::ImageRgba8(imageops::blur(&img.to_rgba8(), sigma))
}

pub fn brightness_image(img: DynamicImage, value: i32) -> DynamicImage {
    DynamicImage::ImageRgba8(imageops::brighten(&img.to_rgba8(), value))
}

pub fn contrast_image(img: DynamicImage, value: f32) -> DynamicImage {
    DynamicImage::ImageRgba8(imageops::contrast(&img.to_rgba8(), value))
}
