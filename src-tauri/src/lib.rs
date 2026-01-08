use image::ImageFormat;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub mod converters;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub output_path: Option<String>,
    pub error: Option<String>,
    pub data: Option<String>, // Base64 encoded data for preview
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub file_type: String,
}

impl FileInfo {
    pub fn from_path(path: &str) -> Result<Self, String> {
        let path_obj = Path::new(path);
        let metadata = fs::metadata(path).map_err(|e| e.to_string())?;
        
        let name = path_obj
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let extension = path_obj
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        let file_type = get_file_type(&extension);
        
        Ok(FileInfo {
            path: path.to_string(),
            name,
            extension,
            size: metadata.len(),
            file_type,
        })
    }
}

pub fn get_file_type(extension: &str) -> String {
    match extension.to_lowercase().as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "tiff" | "tif" | "webp" | "avif" => "image".to_string(),
        "pdf" => "pdf".to_string(),
        "txt" | "md" | "markdown" | "rst" | "log" => "text".to_string(),
        "json" | "xml" | "yaml" | "yml" | "toml" | "ini" | "cfg" | "conf" => "config".to_string(),
        "csv" | "tsv" => "data".to_string(),
        "html" | "htm" | "xhtml" => "html".to_string(),
        "css" | "scss" | "sass" | "less" => "style".to_string(),
        "js" | "ts" | "jsx" | "tsx" | "mjs" | "cjs" => "script".to_string(),
        "py" | "rb" | "php" | "java" | "c" | "cpp" | "h" | "hpp" | "rs" | "go" | "swift" => "code".to_string(),
        "doc" | "docx" => "document".to_string(),
        "xls" | "xlsx" => "spreadsheet".to_string(),
        _ => "unknown".to_string(),
    }
}

pub fn get_supported_outputs(file_type: &str, _extension: &str) -> Vec<String> {
    match file_type {
        "image" => vec![
            "png".to_string(),
            "jpg".to_string(),
            "jpeg".to_string(),
            "gif".to_string(),
            "bmp".to_string(),
            "ico".to_string(),
            "webp".to_string(),
        ],
        "pdf" => vec![
            "txt".to_string(),
            "png".to_string(),
            "jpg".to_string(),
        ],
        "text" | "config" | "code" | "script" | "style" => vec![
            "txt".to_string(),
            "md".to_string(),
            "pdf".to_string(),
            "html".to_string(),
        ],
        "html" => vec![
            "txt".to_string(),
            "md".to_string(),
            "pdf".to_string(),
        ],
        "data" => vec![
            "json".to_string(),
            "csv".to_string(),
            "txt".to_string(),
            "html".to_string(),
        ],
        _ => vec!["txt".to_string()],
    }
}

pub fn get_image_format(extension: &str) -> Option<ImageFormat> {
    match extension.to_lowercase().as_str() {
        "png" => Some(ImageFormat::Png),
        "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
        "gif" => Some(ImageFormat::Gif),
        "bmp" => Some(ImageFormat::Bmp),
        "ico" => Some(ImageFormat::Ico),
        "webp" => Some(ImageFormat::WebP),
        "tiff" | "tif" => Some(ImageFormat::Tiff),
        "avif" => Some(ImageFormat::Avif),
        _ => None,
    }
}
