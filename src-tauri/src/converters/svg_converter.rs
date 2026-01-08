use std::fs;
use crate::ConversionResult;
use resvg::tiny_skia;
use resvg::usvg::{Options, Tree};

/// Convert SVG to PNG
pub fn convert_svg_to_png(input_path: &str, output_path: &str, scale: Option<f32>) -> ConversionResult {
    let svg_data = match fs::read(input_path) {
        Ok(data) => data,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read SVG file: {}", e)),
                data: None,
            };
        }
    };

    let opt = Options::default();
    
    let tree = match Tree::from_data(&svg_data, &opt) {
        Ok(t) => t,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse SVG: {}", e)),
                data: None,
            };
        }
    };

    let scale_factor = scale.unwrap_or(1.0);
    let size = tree.size();
    let width = (size.width() * scale_factor) as u32;
    let height = (size.height() * scale_factor) as u32;

    if width == 0 || height == 0 {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("Invalid SVG dimensions".to_string()),
            data: None,
        };
    }

    let mut pixmap = match tiny_skia::Pixmap::new(width, height) {
        Some(p) => p,
        None => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some("Failed to create image buffer".to_string()),
                data: None,
            };
        }
    };

    // Fill with white background
    pixmap.fill(tiny_skia::Color::WHITE);

    let transform = tiny_skia::Transform::from_scale(scale_factor, scale_factor);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    match pixmap.save_png(output_path) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to save PNG: {}", e)),
            data: None,
        },
    }
}

/// Convert SVG to JPEG
pub fn convert_svg_to_jpg(input_path: &str, output_path: &str, scale: Option<f32>, quality: Option<u8>) -> ConversionResult {
    let svg_data = match fs::read(input_path) {
        Ok(data) => data,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read SVG file: {}", e)),
                data: None,
            };
        }
    };

    let opt = Options::default();
    
    let tree = match Tree::from_data(&svg_data, &opt) {
        Ok(t) => t,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse SVG: {}", e)),
                data: None,
            };
        }
    };

    let scale_factor = scale.unwrap_or(1.0);
    let size = tree.size();
    let width = (size.width() * scale_factor) as u32;
    let height = (size.height() * scale_factor) as u32;

    if width == 0 || height == 0 {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("Invalid SVG dimensions".to_string()),
            data: None,
        };
    }

    let mut pixmap = match tiny_skia::Pixmap::new(width, height) {
        Some(p) => p,
        None => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some("Failed to create image buffer".to_string()),
                data: None,
            };
        }
    };

    // Fill with white background (important for JPEG)
    pixmap.fill(tiny_skia::Color::WHITE);

    let transform = tiny_skia::Transform::from_scale(scale_factor, scale_factor);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // Convert RGBA to RGB for JPEG
    let rgba_data = pixmap.data();
    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
    for chunk in rgba_data.chunks(4) {
        rgb_data.push(chunk[0]); // R
        rgb_data.push(chunk[1]); // G
        rgb_data.push(chunk[2]); // B
    }

    // Use image crate to save as JPEG
    let img = image::RgbImage::from_raw(width, height, rgb_data);
    match img {
        Some(img) => {
            let quality_val = quality.unwrap_or(85);
            let mut output_file = match std::fs::File::create(output_path) {
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
            
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output_file, quality_val);
            match img.write_with_encoder(encoder) {
                Ok(_) => ConversionResult {
                    success: true,
                    output_path: Some(output_path.to_string()),
                    error: None,
                    data: None,
                },
                Err(e) => ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("Failed to save JPEG: {}", e)),
                    data: None,
                },
            }
        }
        None => ConversionResult {
            success: false,
            output_path: None,
            error: Some("Failed to create RGB image".to_string()),
            data: None,
        },
    }
}

/// Get SVG information
pub fn get_svg_info(input_path: &str) -> Result<SvgInfo, String> {
    let svg_data = fs::read(input_path).map_err(|e| format!("Failed to read SVG: {}", e))?;
    
    let opt = Options::default();
    let tree = Tree::from_data(&svg_data, &opt).map_err(|e| format!("Failed to parse SVG: {}", e))?;
    
    let size = tree.size();
    
    Ok(SvgInfo {
        width: size.width(),
        height: size.height(),
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SvgInfo {
    pub width: f32,
    pub height: f32,
}
