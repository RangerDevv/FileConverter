use std::process::Command;
use std::path::PathBuf;
use std::env;
use crate::ConversionResult;

/// Audio formats supported
const AUDIO_FORMATS: &[&str] = &["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma"];

/// Video formats supported
const VIDEO_FORMATS: &[&str] = &["mp4", "mkv", "avi", "mov", "webm", "gif"];

/// Get the path to the bundled ffmpeg binary
fn get_ffmpeg_path() -> PathBuf {
    // In development, use the binaries folder directly
    // In production, Tauri places sidecar binaries next to the executable
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();
    
    // Try bundled binary first (with platform-specific suffix)
    #[cfg(target_os = "linux")]
    let binary_name = "ffmpeg-x86_64-unknown-linux-gnu";
    #[cfg(target_os = "macos")]
    let binary_name = "ffmpeg-x86_64-apple-darwin";
    #[cfg(target_os = "windows")]
    let binary_name = "ffmpeg-x86_64-pc-windows-msvc.exe";
    
    let bundled_path = exe_dir.join(binary_name);
    
    if bundled_path.exists() {
        return bundled_path;
    }
    
    // Try development path
    let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("binaries")
        .join(binary_name);
    
    if dev_path.exists() {
        return dev_path;
    }
    
    // Fall back to system ffmpeg
    PathBuf::from("ffmpeg")
}

/// Get the path to the bundled ffprobe binary
fn get_ffprobe_path() -> PathBuf {
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();
    
    #[cfg(target_os = "linux")]
    let binary_name = "ffprobe-x86_64-unknown-linux-gnu";
    #[cfg(target_os = "macos")]
    let binary_name = "ffprobe-x86_64-apple-darwin";
    #[cfg(target_os = "windows")]
    let binary_name = "ffprobe-x86_64-pc-windows-msvc.exe";
    
    let bundled_path = exe_dir.join(binary_name);
    
    if bundled_path.exists() {
        return bundled_path;
    }
    
    // Try development path
    let dev_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("binaries")
        .join(binary_name);
    
    if dev_path.exists() {
        return dev_path;
    }
    
    // Fall back to system ffprobe
    PathBuf::from("ffprobe")
}

/// Check if ffmpeg is available (bundled or system)
pub fn is_ffmpeg_available() -> bool {
    let ffmpeg_path = get_ffmpeg_path();
    Command::new(&ffmpeg_path)
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Convert audio file to another audio format using ffmpeg
pub fn convert_audio(input_path: &str, output_path: &str, output_format: &str) -> ConversionResult {
    let ffmpeg_path = get_ffmpeg_path();
    
    if !is_ffmpeg_available() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("ffmpeg is not available. The bundled ffmpeg binary may be missing.".to_string()),
            data: None,
        };
    }

    let mut args = vec![
        "-i", input_path,
        "-y", // Overwrite output file
    ];

    // Add format-specific options
    match output_format {
        "mp3" => {
            args.extend(&["-codec:a", "libmp3lame", "-qscale:a", "2"]);
        }
        "wav" => {
            args.extend(&["-codec:a", "pcm_s16le"]);
        }
        "ogg" => {
            args.extend(&["-codec:a", "libvorbis", "-qscale:a", "5"]);
        }
        "flac" => {
            args.extend(&["-codec:a", "flac"]);
        }
        "aac" | "m4a" => {
            args.extend(&["-codec:a", "aac", "-b:a", "192k"]);
        }
        _ => {}
    }

    args.push(output_path);

    let result = Command::new(&ffmpeg_path)
        .args(&args)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                ConversionResult {
                    success: true,
                    output_path: Some(output_path.to_string()),
                    error: None,
                    data: None,
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("ffmpeg error: {}", stderr)),
                    data: None,
                }
            }
        }
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to run ffmpeg: {}", e)),
            data: None,
        },
    }
}

/// Convert video file to another video format using ffmpeg
pub fn convert_video(input_path: &str, output_path: &str, output_format: &str) -> ConversionResult {
    let ffmpeg_path = get_ffmpeg_path();
    
    if !is_ffmpeg_available() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("ffmpeg is not available. The bundled ffmpeg binary may be missing.".to_string()),
            data: None,
        };
    }

    let mut args = vec![
        "-i", input_path,
        "-y", // Overwrite output file
    ];

    // Add format-specific options
    match output_format {
        "mp4" => {
            args.extend(&["-codec:v", "libx264", "-preset", "medium", "-crf", "23", "-codec:a", "aac"]);
        }
        "mkv" => {
            args.extend(&["-codec:v", "libx264", "-preset", "medium", "-crf", "23", "-codec:a", "aac"]);
        }
        "webm" => {
            args.extend(&["-codec:v", "libvpx-vp9", "-crf", "30", "-b:v", "0", "-codec:a", "libopus"]);
        }
        "avi" => {
            args.extend(&["-codec:v", "mpeg4", "-qscale:v", "5", "-codec:a", "libmp3lame"]);
        }
        "mov" => {
            args.extend(&["-codec:v", "libx264", "-preset", "medium", "-crf", "23", "-codec:a", "aac"]);
        }
        "gif" => {
            // For GIF, we need special handling - create a palette first
            args.extend(&["-vf", "fps=10,scale=480:-1:flags=lanczos", "-loop", "0"]);
        }
        _ => {}
    }

    args.push(output_path);

    let result = Command::new(&ffmpeg_path)
        .args(&args)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                ConversionResult {
                    success: true,
                    output_path: Some(output_path.to_string()),
                    error: None,
                    data: None,
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("ffmpeg error: {}", stderr)),
                    data: None,
                }
            }
        }
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to run ffmpeg: {}", e)),
            data: None,
        },
    }
}

/// Extract audio from video file
pub fn extract_audio_from_video(input_path: &str, output_path: &str, audio_format: &str) -> ConversionResult {
    let ffmpeg_path = get_ffmpeg_path();
    
    if !is_ffmpeg_available() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("ffmpeg is not available. The bundled ffmpeg binary may be missing.".to_string()),
            data: None,
        };
    }

    let mut args = vec![
        "-i", input_path,
        "-vn", // No video
        "-y",
    ];

    // Add audio codec options
    match audio_format {
        "mp3" => {
            args.extend(&["-codec:a", "libmp3lame", "-qscale:a", "2"]);
        }
        "wav" => {
            args.extend(&["-codec:a", "pcm_s16le"]);
        }
        "ogg" => {
            args.extend(&["-codec:a", "libvorbis", "-qscale:a", "5"]);
        }
        "flac" => {
            args.extend(&["-codec:a", "flac"]);
        }
        "aac" | "m4a" => {
            args.extend(&["-codec:a", "aac", "-b:a", "192k"]);
        }
        _ => {}
    }

    args.push(output_path);

    let result = Command::new(&ffmpeg_path)
        .args(&args)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                ConversionResult {
                    success: true,
                    output_path: Some(output_path.to_string()),
                    error: None,
                    data: None,
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("ffmpeg error: {}", stderr)),
                    data: None,
                }
            }
        }
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to run ffmpeg: {}", e)),
            data: None,
        },
    }
}

/// Get media file information using ffprobe
pub fn get_media_info(input_path: &str) -> Result<MediaInfo, String> {
    let ffprobe_path = get_ffprobe_path();
    
    let result = Command::new(&ffprobe_path)
        .args([
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            input_path,
        ])
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                let json_str = String::from_utf8_lossy(&output.stdout);
                // Parse basic info from ffprobe output
                let info: serde_json::Value = serde_json::from_str(&json_str)
                    .map_err(|e| format!("Failed to parse ffprobe output: {}", e))?;
                
                let format = info.get("format").ok_or("No format info")?;
                let duration = format.get("duration")
                    .and_then(|d| d.as_str())
                    .and_then(|d| d.parse::<f64>().ok())
                    .unwrap_or(0.0);
                
                let size = format.get("size")
                    .and_then(|s| s.as_str())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0);
                
                let format_name = format.get("format_name")
                    .and_then(|f| f.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                let streams = info.get("streams").and_then(|s| s.as_array());
                let has_video = streams.map(|s| s.iter().any(|stream| {
                    stream.get("codec_type").and_then(|t| t.as_str()) == Some("video")
                })).unwrap_or(false);
                let has_audio = streams.map(|s| s.iter().any(|stream| {
                    stream.get("codec_type").and_then(|t| t.as_str()) == Some("audio")
                })).unwrap_or(false);

                Ok(MediaInfo {
                    duration,
                    size,
                    format: format_name,
                    has_video,
                    has_audio,
                })
            } else {
                Err("ffprobe failed to analyze file".to_string())
            }
        }
        Err(e) => Err(format!("Failed to run ffprobe: {}", e)),
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MediaInfo {
    pub duration: f64,
    pub size: u64,
    pub format: String,
    pub has_video: bool,
    pub has_audio: bool,
}

pub fn is_audio_format(ext: &str) -> bool {
    AUDIO_FORMATS.contains(&ext.to_lowercase().as_str())
}

pub fn is_video_format(ext: &str) -> bool {
    VIDEO_FORMATS.contains(&ext.to_lowercase().as_str())
}
