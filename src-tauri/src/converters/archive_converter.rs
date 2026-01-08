use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
use crate::ConversionResult;
use serde_json::json;

/// List contents of a ZIP archive as JSON
pub fn list_zip_contents(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open ZIP file: {}", e)),
                data: None,
            };
        }
    };

    let mut archive = match zip::ZipArchive::new(file) {
        Ok(a) => a,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read ZIP archive: {}", e)),
                data: None,
            };
        }
    };

    let mut entries = Vec::new();
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            entries.push(json!({
                "name": file.name(),
                "size": file.size(),
                "compressed_size": file.compressed_size(),
                "is_dir": file.is_dir(),
            }));
        }
    }

    let json_output = json!({
        "archive": input_path,
        "total_files": entries.len(),
        "entries": entries,
    });

    let output_str = serde_json::to_string_pretty(&json_output).unwrap();

    match fs::write(output_path, &output_str) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: Some(output_str),
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write JSON file: {}", e)),
            data: None,
        },
    }
}

/// Extract ZIP archive to a directory
pub fn extract_zip(input_path: &str, output_dir: &str) -> ConversionResult {
    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open ZIP file: {}", e)),
                data: None,
            };
        }
    };

    let mut archive = match zip::ZipArchive::new(file) {
        Ok(a) => a,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read ZIP archive: {}", e)),
                data: None,
            };
        }
    };

    // Create output directory
    if let Err(e) = fs::create_dir_all(output_dir) {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to create output directory: {}", e)),
            data: None,
        };
    }

    let mut extracted_count = 0;
    for i in 0..archive.len() {
        let mut file = match archive.by_index(i) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let outpath = Path::new(output_dir).join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&outpath).ok();
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent).ok();
            }
            if let Ok(mut outfile) = File::create(&outpath) {
                std::io::copy(&mut file, &mut outfile).ok();
                extracted_count += 1;
            }
        }
    }

    ConversionResult {
        success: true,
        output_path: Some(output_dir.to_string()),
        error: None,
        data: Some(format!("Extracted {} files", extracted_count)),
    }
}

/// List contents of a TAR archive as JSON
pub fn list_tar_contents(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open TAR file: {}", e)),
                data: None,
            };
        }
    };

    let mut archive = tar::Archive::new(file);
    let entries = match archive.entries() {
        Ok(e) => e,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read TAR archive: {}", e)),
                data: None,
            };
        }
    };

    let mut file_entries = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
            let size = entry.size();
            file_entries.push(json!({
                "name": path,
                "size": size,
            }));
        }
    }

    let json_output = json!({
        "archive": input_path,
        "total_files": file_entries.len(),
        "entries": file_entries,
    });

    let output_str = serde_json::to_string_pretty(&json_output).unwrap();

    match fs::write(output_path, &output_str) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: Some(output_str),
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write JSON file: {}", e)),
            data: None,
        },
    }
}

/// Extract TAR archive to a directory
pub fn extract_tar(input_path: &str, output_dir: &str) -> ConversionResult {
    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open TAR file: {}", e)),
                data: None,
            };
        }
    };

    let mut archive = tar::Archive::new(file);

    // Create output directory
    if let Err(e) = fs::create_dir_all(output_dir) {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to create output directory: {}", e)),
            data: None,
        };
    }

    match archive.unpack(output_dir) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_dir.to_string()),
            error: None,
            data: Some("Archive extracted successfully".to_string()),
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to extract TAR archive: {}", e)),
            data: None,
        },
    }
}

/// List contents of a GZIP-compressed TAR archive (.tar.gz or .tgz)
pub fn list_tar_gz_contents(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open file: {}", e)),
                data: None,
            };
        }
    };

    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);
    
    let entries = match archive.entries() {
        Ok(e) => e,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read archive: {}", e)),
                data: None,
            };
        }
    };

    let mut file_entries = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path().map(|p| p.to_string_lossy().to_string()).unwrap_or_default();
            let size = entry.size();
            file_entries.push(json!({
                "name": path,
                "size": size,
            }));
        }
    }

    let json_output = json!({
        "archive": input_path,
        "total_files": file_entries.len(),
        "entries": file_entries,
    });

    let output_str = serde_json::to_string_pretty(&json_output).unwrap();

    match fs::write(output_path, &output_str) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: Some(output_str),
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write JSON file: {}", e)),
            data: None,
        },
    }
}

/// Extract GZIP-compressed TAR archive (.tar.gz or .tgz)
pub fn extract_tar_gz(input_path: &str, output_dir: &str) -> ConversionResult {
    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open file: {}", e)),
                data: None,
            };
        }
    };

    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(decoder);

    // Create output directory
    if let Err(e) = fs::create_dir_all(output_dir) {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to create output directory: {}", e)),
            data: None,
        };
    }

    match archive.unpack(output_dir) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_dir.to_string()),
            error: None,
            data: Some("Archive extracted successfully".to_string()),
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to extract archive: {}", e)),
            data: None,
        },
    }
}

/// Decompress a single GZIP file (.gz)
pub fn decompress_gz(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open file: {}", e)),
                data: None,
            };
        }
    };

    let mut decoder = flate2::read::GzDecoder::new(file);
    let mut output = Vec::new();

    if let Err(e) = decoder.read_to_end(&mut output) {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to decompress file: {}", e)),
            data: None,
        };
    }

    match fs::write(output_path, &output) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: Some(format!("Decompressed {} bytes", output.len())),
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write output file: {}", e)),
            data: None,
        },
    }
}
