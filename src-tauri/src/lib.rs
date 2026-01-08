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
        // Images
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "ico" | "tiff" | "tif" | "webp" | "avif" => "image".to_string(),
        // SVG (vector)
        "svg" => "svg".to_string(),
        // PDF
        "pdf" => "pdf".to_string(),
        // Text
        "txt" | "log" => "text".to_string(),
        // Markdown
        "md" | "markdown" | "rst" => "markdown".to_string(),
        // Config/Data formats
        "json" => "json".to_string(),
        "xml" => "xml".to_string(),
        "yaml" | "yml" => "yaml".to_string(),
        "toml" => "toml".to_string(),
        "ini" | "cfg" | "conf" => "config".to_string(),
        // Data files
        "csv" | "tsv" => "data".to_string(),
        // HTML
        "html" | "htm" | "xhtml" => "html".to_string(),
        // Style
        "css" | "scss" | "sass" | "less" => "style".to_string(),
        // Script
        "js" | "ts" | "jsx" | "tsx" | "mjs" | "cjs" => "script".to_string(),
        // Code
        "py" | "rb" | "php" | "java" | "c" | "cpp" | "h" | "hpp" | "rs" | "go" | "swift" => "code".to_string(),
        // Documents
        "doc" | "docx" => "docx".to_string(),
        "rtf" => "rtf".to_string(),
        "epub" => "epub".to_string(),
        "odt" => "odt".to_string(),
        // Spreadsheets
        "xls" | "xlsx" => "xlsx".to_string(),
        "ods" => "ods".to_string(),
        // Archives
        "zip" => "zip".to_string(),
        "tar" => "tar".to_string(),
        "gz" | "tgz" => "gzip".to_string(),
        // Audio
        "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" | "wma" => "audio".to_string(),
        // Video
        "mp4" | "mkv" | "avi" | "mov" | "webm" => "video".to_string(),
        _ => "unknown".to_string(),
    }
}

pub fn get_supported_outputs(file_type: &str, _extension: &str) -> Vec<String> {
    match file_type {
        // Images
        "image" => vec![
            "png".to_string(),
            "jpg".to_string(),
            "jpeg".to_string(),
            "gif".to_string(),
            "bmp".to_string(),
            "ico".to_string(),
            "webp".to_string(),
        ],
        // SVG
        "svg" => vec![
            "png".to_string(),
            "jpg".to_string(),
        ],
        // PDF
        "pdf" => vec![
            "txt".to_string(),
            "png".to_string(),
            "jpg".to_string(),
        ],
        // Text
        "text" | "config" | "code" | "script" | "style" => vec![
            "txt".to_string(),
            "md".to_string(),
            "pdf".to_string(),
            "html".to_string(),
        ],
        // Markdown
        "markdown" => vec![
            "html".to_string(),
            "pdf".to_string(),
            "txt".to_string(),
        ],
        // HTML
        "html" => vec![
            "txt".to_string(),
            "md".to_string(),
            "pdf".to_string(),
        ],
        // JSON
        "json" => vec![
            "yaml".to_string(),
            "toml".to_string(),
            "xml".to_string(),
            "csv".to_string(),
            "txt".to_string(),
        ],
        // YAML
        "yaml" => vec![
            "json".to_string(),
            "toml".to_string(),
            "txt".to_string(),
        ],
        // TOML
        "toml" => vec![
            "json".to_string(),
            "yaml".to_string(),
            "txt".to_string(),
        ],
        // XML
        "xml" => vec![
            "json".to_string(),
            "txt".to_string(),
        ],
        // Data (CSV/TSV)
        "data" => vec![
            "json".to_string(),
            "csv".to_string(),
            "txt".to_string(),
            "html".to_string(),
        ],
        // Documents (DOCX)
        "docx" => vec![
            "txt".to_string(),
            "html".to_string(),
            "pdf".to_string(),
        ],
        // RTF
        "rtf" => vec![
            "txt".to_string(),
            "html".to_string(),
        ],
        // EPUB
        "epub" => vec![
            "txt".to_string(),
            "html".to_string(),
        ],
        // ODT
        "odt" => vec![
            "txt".to_string(),
            "pdf".to_string(),
        ],
        // Spreadsheets (XLSX/XLS)
        "xlsx" => vec![
            "csv".to_string(),
            "json".to_string(),
            "html".to_string(),
        ],
        // ODS
        "ods" => vec![
            "csv".to_string(),
            "json".to_string(),
            "html".to_string(),
        ],
        // Archives
        "zip" | "tar" | "gzip" => vec![
            "json".to_string(),  // List contents
            "folder".to_string(), // Extract
        ],
        // Audio
        "audio" => vec![
            "mp3".to_string(),
            "wav".to_string(),
            "ogg".to_string(),
            "flac".to_string(),
            "aac".to_string(),
            "m4a".to_string(),
        ],
        // Video
        "video" => vec![
            "mp4".to_string(),
            "mkv".to_string(),
            "webm".to_string(),
            "avi".to_string(),
            "mov".to_string(),
            "gif".to_string(),
            "mp3".to_string(), // Extract audio
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
