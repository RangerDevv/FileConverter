#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use file_converter_lib::converters::*;
use file_converter_lib::*;
use std::fs;
use std::path::Path;

#[tauri::command]
fn get_file_info(path: String) -> Result<FileInfo, String> {
    FileInfo::from_path(&path)
}

#[tauri::command]
fn get_supported_formats(file_type: String, extension: String) -> Vec<String> {
    get_supported_outputs(&file_type, &extension)
}

#[tauri::command]
fn convert_file(
    input_path: String,
    output_format: String,
    output_dir: String,
    options: Option<ConvertOptions>,
) -> ConversionResult {
    let file_info = match FileInfo::from_path(&input_path) {
        Ok(info) => info,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(e),
                data: None,
            }
        }
    };

    // Generate output filename
    let stem = Path::new(&input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    
    let output_path = Path::new(&output_dir)
        .join(format!("{}_converted.{}", stem, output_format))
        .to_string_lossy()
        .to_string();

    // Ensure output directory exists
    if let Err(e) = fs::create_dir_all(&output_dir) {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to create output directory: {}", e)),
            data: None,
        };
    }

    match file_info.file_type.as_str() {
        "image" => {
            let quality = options.as_ref().and_then(|o| o.quality);
            convert_image(&input_path, &output_format, &output_path, quality)
        }
        "pdf" => match output_format.as_str() {
            "txt" => extract_pdf_text(&input_path, &output_path),
            "png" | "jpg" | "jpeg" => convert_pdf_to_image(&input_path, &output_path, &output_format),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported PDF conversion to {}", output_format)),
                data: None,
            },
        },
        "text" | "config" | "code" | "script" | "style" => match output_format.as_str() {
            "pdf" => convert_text_to_pdf(&input_path, &output_path),
            "html" => convert_text_to_html(&input_path, &output_path),
            "md" => convert_text_to_markdown(&input_path, &output_path),
            "txt" => convert_to_plain_text(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported text conversion to {}", output_format)),
                data: None,
            },
        },
        "html" => match output_format.as_str() {
            "txt" => convert_to_plain_text(&input_path, &output_path),
            "md" => convert_text_to_markdown(&input_path, &output_path),
            "pdf" => convert_text_to_pdf(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported HTML conversion to {}", output_format)),
                data: None,
            },
        },
        "data" => match (file_info.extension.as_str(), output_format.as_str()) {
            ("csv" | "tsv", "json") => convert_csv_to_json(&input_path, &output_path),
            ("csv" | "tsv", "html") => convert_csv_to_html(&input_path, &output_path),
            ("json", "csv") => convert_json_to_csv(&input_path, &output_path),
            ("csv" | "tsv", "txt") => convert_to_plain_text(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!(
                    "Unsupported data conversion from {} to {}",
                    file_info.extension, output_format
                )),
                data: None,
            },
        },
        _ => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Unsupported file type: {}", file_info.file_type)),
            data: None,
        },
    }
}

#[tauri::command]
fn resize_image_cmd(
    input_path: String,
    output_path: String,
    width: u32,
    height: u32,
    maintain_aspect: bool,
) -> ConversionResult {
    resize_image(&input_path, &output_path, width, height, maintain_aspect)
}

#[tauri::command]
fn get_image_preview_cmd(input_path: String, max_size: u32) -> Result<String, String> {
    get_image_preview(&input_path, max_size)
}

#[tauri::command]
fn get_image_info_cmd(input_path: String) -> Result<ImageInfo, String> {
    let (width, height, color_type) = get_image_info(&input_path)?;
    Ok(ImageInfo {
        width,
        height,
        color_type,
    })
}

#[tauri::command]
fn get_pdf_info_cmd(input_path: String) -> Result<PdfInfo, String> {
    get_pdf_info(&input_path)
}

#[tauri::command]
fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn open_file_location(path: String) -> Result<(), String> {
    let dir = Path::new(&path)
        .parent()
        .ok_or("Could not get parent directory")?;
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(dir)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ConvertOptions {
    pub quality: Option<u8>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub color_type: String,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_file_info,
            get_supported_formats,
            convert_file,
            resize_image_cmd,
            get_image_preview_cmd,
            get_image_info_cmd,
            get_pdf_info_cmd,
            read_text_file,
            open_file_location,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
