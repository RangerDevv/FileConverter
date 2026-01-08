use std::fs;
use std::process::Command;
use std::path::Path;
use crate::ConversionResult;

pub fn extract_pdf_text(input_path: &str, output_path: &str) -> ConversionResult {
    let text = match pdf_extract::extract_text(input_path) {
        Ok(t) => t,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to extract PDF text: {}", e)),
                data: None,
            };
        }
    };

    match fs::write(output_path, &text) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: Some(if text.len() > 1000 {
                format!("{}...", &text[..1000])
            } else {
                text
            }),
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write text file: {}", e)),
            data: None,
        },
    }
}

pub fn get_pdf_info(input_path: &str) -> Result<PdfInfo, String> {
    let doc = lopdf::Document::load(input_path).map_err(|e| e.to_string())?;
    
    let page_count = doc.get_pages().len();
    
    // Get PDF version
    let version = doc.version.clone();
    
    Ok(PdfInfo {
        page_count,
        version,
    })
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PdfInfo {
    pub page_count: usize,
    pub version: String,
}

/// Convert PDF to PNG images using pdftoppm (poppler-utils)
/// This creates one PNG per page, with the output path used as a base name
pub fn convert_pdf_to_image(input_path: &str, output_path: &str, format: &str) -> ConversionResult {
    let output_base = Path::new(output_path);
    let output_dir = output_base.parent().unwrap_or(Path::new("."));
    let stem = output_base.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    
    // Build the output prefix (pdftoppm adds page numbers automatically)
    let output_prefix = output_dir.join(stem);
    
    // Try using pdftoppm (from poppler-utils) which is commonly available on Linux
    let format_flag = match format {
        "png" => "-png",
        "jpg" | "jpeg" => "-jpeg",
        _ => "-png",
    };
    
    let result = Command::new("pdftoppm")
        .args([
            format_flag,
            "-r", "150", // 150 DPI resolution
            input_path,
            output_prefix.to_str().unwrap_or("output"),
        ])
        .output();
    
    match result {
        Ok(output) => {
            if output.status.success() {
                // pdftoppm creates files like output-1.png, output-2.png, etc.
                // Find the generated files
                let generated_files: Vec<String> = std::fs::read_dir(output_dir)
                    .map(|entries| {
                        entries
                            .filter_map(|e| e.ok())
                            .filter(|e| {
                                e.file_name()
                                    .to_str()
                                    .map(|name| name.starts_with(&format!("{}-", stem)) && name.ends_with(&format!(".{}", format)))
                                    .unwrap_or(false)
                            })
                            .map(|e| e.path().to_string_lossy().to_string())
                            .collect()
                    })
                    .unwrap_or_default();
                
                if generated_files.is_empty() {
                    // Maybe it's a single page PDF, check for direct output
                    let single_page = format!("{}-1.{}", output_prefix.to_str().unwrap_or(""), format);
                    if Path::new(&single_page).exists() {
                        // Rename to expected output path for single page
                        if let Err(e) = std::fs::rename(&single_page, output_path) {
                            return ConversionResult {
                                success: false,
                                output_path: None,
                                error: Some(format!("Failed to rename output file: {}", e)),
                                data: None,
                            };
                        }
                        return ConversionResult {
                            success: true,
                            output_path: Some(output_path.to_string()),
                            error: None,
                            data: Some("Converted 1 page".to_string()),
                        };
                    }
                    
                    ConversionResult {
                        success: false,
                        output_path: None,
                        error: Some("No output files were generated".to_string()),
                        data: None,
                    }
                } else {
                    // For multi-page PDFs, return the first page path
                    // and mention how many pages were converted
                    let first_file = generated_files.first().cloned();
                    ConversionResult {
                        success: true,
                        output_path: first_file,
                        error: None,
                        data: Some(format!("Converted {} page(s) to {}", generated_files.len(), format.to_uppercase())),
                    }
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("pdftoppm failed: {}", stderr)),
                    data: None,
                }
            }
        }
        Err(e) => {
            // pdftoppm not found, suggest installation
            if e.kind() == std::io::ErrorKind::NotFound {
                ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some("PDF to image conversion requires 'poppler-utils'. Install with: sudo apt install poppler-utils".to_string()),
                    data: None,
                }
            } else {
                ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("Failed to run pdftoppm: {}", e)),
                    data: None,
                }
            }
        }
    }
}
