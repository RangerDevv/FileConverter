use std::fs;
use crate::ConversionResult;
use docx_rs::*;

/// Extract text from DOCX file
pub fn convert_docx_to_txt(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::read(input_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read DOCX file: {}", e)),
                data: None,
            };
        }
    };

    let doc = match read_docx(&file) {
        Ok(d) => d,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse DOCX file: {}", e)),
                data: None,
            };
        }
    };

    let mut text = String::new();
    
    // Extract text from document body
    for child in doc.document.children {
        extract_text_from_element(&child, &mut text);
    }

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
            error: Some(format!("Failed to write output file: {}", e)),
            data: None,
        },
    }
}

fn extract_text_from_element(element: &DocumentChild, text: &mut String) {
    match element {
        DocumentChild::Paragraph(p) => {
            for child in &p.children {
                match child {
                    ParagraphChild::Run(run) => {
                        for run_child in &run.children {
                            if let RunChild::Text(t) = run_child {
                                text.push_str(&t.text);
                            }
                        }
                    }
                    ParagraphChild::Hyperlink(link) => {
                        for pc in &link.children {
                            if let ParagraphChild::Run(r) = pc {
                                for run_child in &r.children {
                                    if let RunChild::Text(t) = run_child {
                                        text.push_str(&t.text);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            text.push('\n');
        }
        DocumentChild::Table(table) => {
            for row in &table.rows {
                let TableChild::TableRow(tr) = row;
                for tc in &tr.cells {
                    let TableRowChild::TableCell(cell) = tc;
                    for cell_content in &cell.children {
                        if let TableCellContent::Paragraph(p) = cell_content {
                            for child in &p.children {
                                if let ParagraphChild::Run(run) = child {
                                    for run_child in &run.children {
                                        if let RunChild::Text(t) = run_child {
                                            text.push_str(&t.text);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    text.push('\t');
                }
                text.push('\n');
            }
        }
        _ => {}
    }
}

/// Convert DOCX to HTML
pub fn convert_docx_to_html(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::read(input_path) {
        Ok(f) => f,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read DOCX file: {}", e)),
                data: None,
            };
        }
    };

    let doc = match read_docx(&file) {
        Ok(d) => d,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse DOCX file: {}", e)),
                data: None,
            };
        }
    };

    let filename = std::path::Path::new(input_path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("Document");

    let mut html = format!(
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
        }}
        table {{
            border-collapse: collapse;
            width: 100%;
            margin: 20px 0;
        }}
        th, td {{
            border: 1px solid #ddd;
            padding: 8px;
            text-align: left;
        }}
        th {{ background: #f5f5f5; }}
    </style>
</head>
<body>
"#,
        filename
    );

    for child in doc.document.children {
        element_to_html(&child, &mut html);
    }

    html.push_str("</body>\n</html>");

    match fs::write(output_path, &html) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write HTML file: {}", e)),
            data: None,
        },
    }
}

fn element_to_html(element: &DocumentChild, html: &mut String) {
    match element {
        DocumentChild::Paragraph(p) => {
            html.push_str("<p>");
            for child in &p.children {
                if let ParagraphChild::Run(run) = child {
                    let has_bold = run.run_property.bold.is_some();
                    let has_italic = run.run_property.italic.is_some();
                    
                    if has_bold { html.push_str("<strong>"); }
                    if has_italic { html.push_str("<em>"); }
                    
                    for run_child in &run.children {
                        if let RunChild::Text(t) = run_child {
                            html.push_str(&escape_html(&t.text));
                        }
                    }
                    
                    if has_italic { html.push_str("</em>"); }
                    if has_bold { html.push_str("</strong>"); }
                }
            }
            html.push_str("</p>\n");
        }
        DocumentChild::Table(table) => {
            html.push_str("<table>\n");
            for row in &table.rows {
                let TableChild::TableRow(tr) = row;
                html.push_str("<tr>\n");
                for tc in &tr.cells {
                    let TableRowChild::TableCell(cell) = tc;
                    html.push_str("<td>");
                    for cell_content in &cell.children {
                        if let TableCellContent::Paragraph(p) = cell_content {
                            for child in &p.children {
                                if let ParagraphChild::Run(run) = child {
                                    for run_child in &run.children {
                                        if let RunChild::Text(t) = run_child {
                                            html.push_str(&escape_html(&t.text));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    html.push_str("</td>\n");
                }
                html.push_str("</tr>\n");
            }
            html.push_str("</table>\n");
        }
        _ => {}
    }
}

/// Extract text from EPUB file
pub fn convert_epub_to_txt(input_path: &str, output_path: &str) -> ConversionResult {
    let mut doc = match epub::doc::EpubDoc::new(input_path) {
        Ok(d) => d,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open EPUB file: {}", e)),
                data: None,
            };
        }
    };

    let mut text = String::new();
    
    // Get title if available
    if let Some(title) = doc.mdata("title") {
        text.push_str(&format!("Title: {}\n", title.value));
    }
    if let Some(author) = doc.mdata("creator") {
        text.push_str(&format!("Author: {}\n", author.value));
    }
    text.push_str("\n---\n\n");
    
    // Extract text from each spine item
    let spine_ids: Vec<String> = doc.spine.iter().map(|item| item.idref.clone()).collect();
    for id in spine_ids {
        if let Some((content, _mime)) = doc.get_resource(&id) {
            let content_str = String::from_utf8_lossy(&content);
            let plain_text = strip_html_tags(&content_str);
            if !plain_text.trim().is_empty() {
                text.push_str(&plain_text);
                text.push_str("\n\n");
            }
        }
    }

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
            error: Some(format!("Failed to write output file: {}", e)),
            data: None,
        },
    }
}

/// Convert EPUB to HTML (single HTML file with all content)
pub fn convert_epub_to_html(input_path: &str, output_path: &str) -> ConversionResult {
    let mut doc = match epub::doc::EpubDoc::new(input_path) {
        Ok(d) => d,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to open EPUB file: {}", e)),
                data: None,
            };
        }
    };

    let title = doc.mdata("title")
        .map(|m| m.value.to_string())
        .unwrap_or_else(|| "Document".to_string());
    let author = doc.mdata("creator")
        .map(|m| m.value.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let mut html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{
            font-family: Georgia, 'Times New Roman', serif;
            max-width: 800px;
            margin: 40px auto;
            padding: 20px;
            line-height: 1.8;
            color: #333;
        }}
        h1, h2, h3 {{ color: #222; }}
        .title {{ text-align: center; margin-bottom: 10px; }}
        .author {{ text-align: center; color: #666; margin-bottom: 40px; }}
        hr {{ border: none; border-top: 1px solid #ddd; margin: 40px 0; }}
        img {{ max-width: 100%; height: auto; }}
    </style>
</head>
<body>
    <h1 class="title">{}</h1>
    <p class="author">by {}</p>
    <hr>
"#,
        escape_html(&title),
        escape_html(&title),
        escape_html(&author)
    );

    // Extract content from each spine item
    let spine_ids: Vec<String> = doc.spine.iter().map(|item| item.idref.clone()).collect();
    for id in spine_ids {
        if let Some((content, _mime)) = doc.get_resource(&id) {
            let content_str = String::from_utf8_lossy(&content);
            // Extract body content only
            if let Some(body_start) = content_str.find("<body") {
                if let Some(body_end) = content_str.rfind("</body>") {
                    let body_tag_end = content_str[body_start..].find('>').unwrap_or(0) + body_start + 1;
                    let body_content = &content_str[body_tag_end..body_end];
                    html.push_str(body_content);
                    html.push_str("<hr>\n");
                }
            }
        }
    }

    html.push_str("</body>\n</html>");

    match fs::write(output_path, &html) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write HTML file: {}", e)),
            data: None,
        },
    }
}

/// Convert RTF to plain text (basic implementation)
pub fn convert_rtf_to_txt(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read RTF file: {}", e)),
                data: None,
            };
        }
    };

    // Basic RTF to text conversion - strip RTF control words
    let text = strip_rtf(&content);

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
            error: Some(format!("Failed to write output file: {}", e)),
            data: None,
        },
    }
}

/// Strip RTF control words and extract plain text
fn strip_rtf(rtf: &str) -> String {
    let mut result = String::new();
    let mut chars = rtf.chars().peekable();
    let mut skip_group: i32 = 0;
    
    while let Some(c) = chars.next() {
        match c {
            '{' => skip_group += 1,
            '}' => skip_group = skip_group.saturating_sub(1),
            '\\' => {
                // Handle RTF control word
                let mut word = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphabetic() {
                        word.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                
                // Skip numeric parameter
                while let Some(&next) = chars.peek() {
                    if next.is_numeric() || next == '-' {
                        chars.next();
                    } else {
                        break;
                    }
                }
                
                // Skip optional space after control word
                if let Some(&' ') = chars.peek() {
                    chars.next();
                }
                
                // Handle special control words
                match word.as_str() {
                    "par" | "line" => result.push('\n'),
                    "tab" => result.push('\t'),
                    "'" => {
                        // Hex character - skip for simplicity
                        chars.next();
                        chars.next();
                    }
                    _ => {}
                }
            }
            _ if skip_group <= 1 && c != '\r' && c != '\n' => {
                result.push(c);
            }
            _ => {}
        }
    }
    
    result.trim().to_string()
}

/// Strip HTML tags from content
fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    let mut last_was_space = false;
    
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => {
                if c.is_whitespace() {
                    if !last_was_space {
                        result.push(' ');
                        last_was_space = true;
                    }
                } else {
                    result.push(c);
                    last_was_space = false;
                }
            }
            _ => {}
        }
    }
    
    result
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
