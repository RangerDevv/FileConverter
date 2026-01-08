use image::{ImageFormat, GenericImageView};
use std::fs::File;
use std::io::{BufWriter, Cursor};
use std::path::Path;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::{ConversionResult, get_image_format};

pub fn convert_image(
    input_path: &str,
    output_format: &str,
    output_path: &str,
    quality: Option<u8>,
) -> ConversionResult {
    let img = match image::open(input_path) {
        Ok(img) => img,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open image: {}", e)),
                data: None,
            };
        }
    };

    let format = match get_image_format(output_format) {
        Some(f) => f,
        None => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported output format: {}", output_format)),
                data: None,
            };
        }
    };

    // Create output file
    let output_file = match File::create(output_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to create output file: {}", e)),
                data: None,
            };
        }
    };

    let mut writer = BufWriter::new(output_file);

    // Handle JPEG quality
    let result = if format == ImageFormat::Jpeg {
        let quality = quality.unwrap_or(85);
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut writer, quality);
        img.write_with_encoder(encoder)
    } else {
        img.write_to(&mut writer, format)
    };

    match result {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to save image: {}", e)),
            data: None,
        },
    }
}

pub fn resize_image(
    input_path: &str,
    output_path: &str,
    width: u32,
    height: u32,
    maintain_aspect: bool,
) -> ConversionResult {
    let img = match image::open(input_path) {
        Ok(img) => img,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open image: {}", e)),
                data: None,
            };
        }
    };

    let resized = if maintain_aspect {
        img.resize(width, height, image::imageops::FilterType::Lanczos3)
    } else {
        img.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
    };

    let extension = Path::new(output_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");

    let format = get_image_format(extension).unwrap_or(ImageFormat::Png);

    match resized.save_with_format(output_path, format) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to save resized image: {}", e)),
            data: None,
        },
    }
}

pub fn get_image_preview(input_path: &str, max_size: u32) -> Result<String, String> {
    let img = image::open(input_path).map_err(|e| e.to_string())?;
    
    let thumbnail = img.thumbnail(max_size, max_size);
    
    let mut buffer = Cursor::new(Vec::new());
    thumbnail
        .write_to(&mut buffer, ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    
    Ok(BASE64.encode(buffer.get_ref()))
}

pub fn get_image_info(input_path: &str) -> Result<(u32, u32, String), String> {
    let img = image::open(input_path).map_err(|e| e.to_string())?;
    let (width, height) = img.dimensions();
    let color_type = format!("{:?}", img.color());
    Ok((width, height, color_type))
}
