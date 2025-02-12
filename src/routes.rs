use axum::{
    body::Bytes,
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use std::io::Cursor;
use image::{DynamicImage, ImageFormat};
use crate::errors::ImageError;
use crate::manipulations::{
    grayscale_image, invert_image, rotate90_image,
    blur_image, brightness_image, contrast_image,
};

#[derive(Debug, Deserialize)]
pub struct BlurParams {
    pub sigma: f32,
}

#[derive(Debug, Deserialize)]
pub struct BrightnessParams {
    pub value: i32,
}

#[derive(Debug, Deserialize)]
pub struct ContrastParams {
    pub value: f32,
}

fn process_image<F>(
    body: Bytes,
    transform: F,
) -> Result<(StatusCode, HeaderMap, Vec<u8>), ImageError>
where
    F: FnOnce(DynamicImage) -> DynamicImage,
{
    let format = image::guess_format(&body)
        .map_err(|_| ImageError::ImageDecodeError("Unknown or invalid format".to_string()))?;

    let img = image::load_from_memory_with_format(&body, format)?;

    let processed = transform(img).into_rgb8(); // Convert to RGB

    let mut buf = Cursor::new(Vec::new());
    DynamicImage::ImageRgb8((processed)).write_to(&mut buf, format)?;
    let out_bytes = buf.into_inner();

    let mime_type = match format {
        ImageFormat::Png  => "image/png",
        ImageFormat::Jpeg => "image/jpeg",
        ImageFormat::Gif  => "image/gif",
        ImageFormat::WebP => "image/webp",
        _ => "application/octet-stream",
    };
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", mime_type.parse().unwrap());

    Ok((StatusCode::OK, headers, out_bytes))
}


pub async fn grayscale_handler(body: Bytes) -> Result<impl IntoResponse, ImageError> {
    process_image(body, |img| grayscale_image(img))
}

pub async fn invert_handler(body: Bytes) -> Result<impl IntoResponse, ImageError> {
    process_image(body, invert_image)
}

pub async fn rotate90_handler(body: Bytes) -> Result<impl IntoResponse, ImageError> {
    process_image(body, rotate90_image)
}

pub async fn blur_handler(
    Query(params): Query<BlurParams>,
    body: Bytes,
) -> Result<impl IntoResponse, ImageError> {
    process_image(body, |img| blur_image(img, params.sigma))
}

pub async fn brightness_handler(
    Query(params): Query<BrightnessParams>,
    body: Bytes,
) -> Result<impl IntoResponse, ImageError> {
    process_image(body, |img| brightness_image(img, params.value))
}

pub async fn contrast_handler(
    Query(params): Query<ContrastParams>,
    body: Bytes,
) -> Result<impl IntoResponse, ImageError> {
    process_image(body, |img| contrast_image(img, params.value))
}
