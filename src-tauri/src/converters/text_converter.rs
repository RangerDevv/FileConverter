use std::fs;
use crate::ConversionResult;
use printpdf::*;
use encoding_rs::*;

pub fn convert_text_to_pdf(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read(input_path) {
        Ok(bytes) => {
            // Try UTF-8 first, then fallback to other encodings
            let (result, _, _) = UTF_8.decode(&bytes);
            if result.contains('\u{FFFD}') {
                let (result, _, _) = WINDOWS_1252.decode(&bytes);
                result.to_string()
            } else {
                result.to_string()
            }
        }
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read file: {}", e)),
                data: None,
            };
        }
    };

    // Create PDF
    let (doc, page1, layer1) = PdfDocument::new("Converted Document", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Use built-in font
    let font = doc.add_builtin_font(BuiltinFont::Courier).unwrap();

    let font_size = 10.0;
    let line_height = 4.5;
    let margin_top = 280.0;
    let margin_left = 15.0;
    let chars_per_line = 85; // Approximate characters per line with Courier at 10pt

    let mut y_position = margin_top;
    let lines: Vec<&str> = content.lines().collect();
    
    let mut current_layer_ref = current_layer;

    for line in lines {
        // Word wrap long lines
        let wrapped_lines = word_wrap(line, chars_per_line);
        
        for wrapped_line in wrapped_lines {
            if y_position < 20.0 {
                // Create new page
                let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                current_layer_ref = doc.get_page(new_page).get_layer(new_layer);
                y_position = margin_top;
            }

            // Filter out non-printable characters for PDF
            let clean_line: String = wrapped_line
                .chars()
                .filter(|c| c.is_ascii_graphic() || *c == ' ')
                .collect();

            current_layer_ref.use_text(&clean_line, font_size, Mm(margin_left), Mm(y_position), &font);
            y_position -= line_height;
        }
    }

    match doc.save(&mut std::io::BufWriter::new(
        fs::File::create(output_path).unwrap(),
    )) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to save PDF: {}", e)),
            data: None,
        },
    }
}

fn word_wrap(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            if word.len() > max_chars {
                // Break very long words
                let mut remaining = word;
                while remaining.len() > max_chars {
                    lines.push(remaining[..max_chars].to_string());
                    remaining = &remaining[max_chars..];
                }
                current_line = remaining.to_string();
            } else {
                current_line = word.to_string();
            }
        } else if current_line.len() + 1 + word.len() <= max_chars {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() || text.trim().is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

pub fn convert_text_to_html(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read file: {}", e)),
                data: None,
            };
        }
    };

    let filename = std::path::Path::new(input_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Document");

    // Escape HTML entities
    let escaped_content = content
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\n', "<br>\n");

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 800px;
            margin: 40px auto;
            padding: 20px;
            line-height: 1.6;
            color: #333;
            background: #fafafa;
        }}
        pre {{
            background: #f4f4f4;
            padding: 15px;
            border-radius: 5px;
            overflow-x: auto;
        }}
    </style>
</head>
<body>
    <h1>{}</h1>
    <div class="content">
        {}
    </div>
</body>
</html>"#,
        filename, filename, escaped_content
    );

    match fs::write(output_path, html) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write HTML: {}", e)),
            data: None,
        },
    }
}

pub fn convert_text_to_markdown(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read file: {}", e)),
                data: None,
            };
        }
    };

    let filename = std::path::Path::new(input_path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("Document");

    let markdown = format!("# {}\n\n{}", filename, content);

    match fs::write(output_path, markdown) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write Markdown: {}", e)),
            data: None,
        },
    }
}

pub fn convert_to_plain_text(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read file: {}", e)),
                data: None,
            };
        }
    };

    // Strip HTML tags if present
    let plain_text = strip_html_tags(&content);

    match fs::write(output_path, plain_text) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write text: {}", e)),
            data: None,
        },
    }
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut in_script = false;
    let mut in_style = false;

    let chars: Vec<char> = html.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        
        if c == '<' {
            in_tag = true;
            
            // Check for script/style tags
            let remaining: String = chars[i..].iter().take(10).collect();
            if remaining.to_lowercase().starts_with("<script") {
                in_script = true;
            } else if remaining.to_lowercase().starts_with("</script") {
                in_script = false;
            } else if remaining.to_lowercase().starts_with("<style") {
                in_style = true;
            } else if remaining.to_lowercase().starts_with("</style") {
                in_style = false;
            }
        } else if c == '>' {
            in_tag = false;
        } else if !in_tag && !in_script && !in_style {
            result.push(c);
        }
        
        i += 1;
    }

    // Decode common HTML entities
    result
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}
