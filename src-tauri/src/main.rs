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
        "svg" => match output_format.as_str() {
            "png" => convert_svg_to_png(&input_path, &output_path, options.as_ref().and_then(|o| o.scale)),
            "jpg" | "jpeg" => convert_svg_to_jpg(&input_path, &output_path, options.as_ref().and_then(|o| o.scale), options.as_ref().and_then(|o| o.quality)),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported SVG conversion to {}", output_format)),
                data: None,
            },
        },
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
        "markdown" => match output_format.as_str() {
            "html" => convert_markdown_to_html(&input_path, &output_path),
            "pdf" => convert_text_to_pdf(&input_path, &output_path),
            "txt" => convert_to_plain_text(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported Markdown conversion to {}", output_format)),
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
        "json" => match output_format.as_str() {
            "yaml" | "yml" => convert_json_to_yaml(&input_path, &output_path),
            "toml" => convert_json_to_toml(&input_path, &output_path),
            "xml" => convert_json_to_xml(&input_path, &output_path),
            "csv" => convert_json_to_csv(&input_path, &output_path),
            "txt" => convert_to_plain_text(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported JSON conversion to {}", output_format)),
                data: None,
            },
        },
        "yaml" => match output_format.as_str() {
            "json" => convert_yaml_to_json(&input_path, &output_path),
            "txt" => convert_to_plain_text(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported YAML conversion to {}", output_format)),
                data: None,
            },
        },
        "toml" => match output_format.as_str() {
            "json" => convert_toml_to_json(&input_path, &output_path),
            "txt" => convert_to_plain_text(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported TOML conversion to {}", output_format)),
                data: None,
            },
        },
        "xml" => match output_format.as_str() {
            "json" => convert_xml_to_json(&input_path, &output_path),
            "txt" => convert_to_plain_text(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported XML conversion to {}", output_format)),
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
        "docx" => match output_format.as_str() {
            "txt" => convert_docx_to_txt(&input_path, &output_path),
            "html" => convert_docx_to_html(&input_path, &output_path),
            "pdf" => {
                // First convert to HTML, then to PDF via text converter
                let temp_html = format!("{}.temp.html", output_path);
                let html_result = convert_docx_to_html(&input_path, &temp_html);
                if html_result.success {
                    let pdf_result = convert_text_to_pdf(&temp_html, &output_path);
                    let _ = fs::remove_file(&temp_html);
                    pdf_result
                } else {
                    html_result
                }
            }
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported DOCX conversion to {}", output_format)),
                data: None,
            },
        },
        "rtf" => match output_format.as_str() {
            "txt" => convert_rtf_to_txt(&input_path, &output_path),
            "html" => {
                // Convert RTF to TXT first, then wrap in HTML
                let txt_result = convert_rtf_to_txt(&input_path, &output_path);
                if txt_result.success {
                    convert_text_to_html(&output_path, &output_path)
                } else {
                    txt_result
                }
            }
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported RTF conversion to {}", output_format)),
                data: None,
            },
        },
        "epub" => match output_format.as_str() {
            "txt" => convert_epub_to_txt(&input_path, &output_path),
            "html" => convert_epub_to_html(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported EPUB conversion to {}", output_format)),
                data: None,
            },
        },
        "xlsx" => match output_format.as_str() {
            "csv" => convert_xlsx_to_csv(&input_path, &output_path),
            "json" => convert_xlsx_to_json(&input_path, &output_path),
            "html" => convert_xlsx_to_html(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported XLSX conversion to {}", output_format)),
                data: None,
            },
        },
        "ods" => match output_format.as_str() {
            "csv" => convert_ods_to_csv(&input_path, &output_path),
            "json" => convert_ods_to_json(&input_path, &output_path),
            "html" => convert_ods_to_html(&input_path, &output_path),
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported ODS conversion to {}", output_format)),
                data: None,
            },
        },
        "zip" => match output_format.as_str() {
            "json" => list_zip_contents(&input_path, &output_path),
            "folder" => {
                let extract_dir = Path::new(&output_dir)
                    .join(format!("{}_extracted", stem))
                    .to_string_lossy()
                    .to_string();
                extract_zip(&input_path, &extract_dir)
            }
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported ZIP conversion to {}", output_format)),
                data: None,
            },
        },
        "tar" => match output_format.as_str() {
            "json" => list_tar_contents(&input_path, &output_path),
            "folder" => {
                let extract_dir = Path::new(&output_dir)
                    .join(format!("{}_extracted", stem))
                    .to_string_lossy()
                    .to_string();
                extract_tar(&input_path, &extract_dir)
            }
            _ => ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Unsupported TAR conversion to {}", output_format)),
                data: None,
            },
        },
        "gzip" => {
            // Check if it's a .tar.gz or just .gz
            let is_tar_gz = input_path.ends_with(".tar.gz") || input_path.ends_with(".tgz");
            match output_format.as_str() {
                "json" => {
                    if is_tar_gz {
                        list_tar_gz_contents(&input_path, &output_path)
                    } else {
                        ConversionResult {
                            success: false,
                            output_path: None,
                            error: Some("JSON listing only supported for .tar.gz files".to_string()),
                            data: None,
                        }
                    }
                }
                "folder" => {
                    let extract_dir = Path::new(&output_dir)
                        .join(format!("{}_extracted", stem))
                        .to_string_lossy()
                        .to_string();
                    if is_tar_gz {
                        extract_tar_gz(&input_path, &extract_dir)
                    } else {
                        // Single .gz file - decompress
                        let decompressed_name = stem.trim_end_matches(".tar");
                        let decompressed_path = Path::new(&output_dir)
                            .join(decompressed_name)
                            .to_string_lossy()
                            .to_string();
                        decompress_gz(&input_path, &decompressed_path)
                    }
                }
                _ => ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("Unsupported GZIP conversion to {}", output_format)),
                    data: None,
                },
            }
        },
        "audio" => {
            if is_audio_format(&output_format) {
                convert_audio(&input_path, &output_path, &output_format)
            } else {
                ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("Unsupported audio conversion to {}", output_format)),
                    data: None,
                }
            }
        },
        "video" => {
            if is_video_format(&output_format) {
                convert_video(&input_path, &output_path, &output_format)
            } else if is_audio_format(&output_format) {
                // Extract audio from video
                extract_audio_from_video(&input_path, &output_path, &output_format)
            } else {
                ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("Unsupported video conversion to {}", output_format)),
                    data: None,
                }
            }
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
    pub scale: Option<f32>,
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
