use std::fs;
use crate::ConversionResult;
use csv::ReaderBuilder;
use serde_json::{json, Value};

pub fn convert_csv_to_json(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read CSV file: {}", e)),
                data: None,
            };
        }
    };

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let headers: Vec<String> = match reader.headers() {
        Ok(h) => h.iter().map(|s| s.to_string()).collect(),
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read CSV headers: {}", e)),
                data: None,
            };
        }
    };

    let mut records: Vec<Value> = Vec::new();

    for result in reader.records() {
        match result {
            Ok(record) => {
                let mut obj = serde_json::Map::new();
                for (i, field) in record.iter().enumerate() {
                    if i < headers.len() {
                        obj.insert(headers[i].clone(), json!(field));
                    }
                }
                records.push(Value::Object(obj));
            }
            Err(e) => {
                return ConversionResult {
                    success: false,
                    output_path: None,
                    error: Some(format!("Failed to read CSV record: {}", e)),
                    data: None,
                };
            }
        }
    }

    let json_output = match serde_json::to_string_pretty(&records) {
        Ok(j) => j,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to create JSON: {}", e)),
                data: None,
            };
        }
    };

    match fs::write(output_path, &json_output) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write JSON file: {}", e)),
            data: None,
        },
    }
}

pub fn convert_json_to_csv(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read JSON file: {}", e)),
                data: None,
            };
        }
    };

    let json_data: Value = match serde_json::from_str(&content) {
        Ok(j) => j,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse JSON: {}", e)),
                data: None,
            };
        }
    };

    let array = match json_data.as_array() {
        Some(a) => a,
        None => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some("JSON must be an array of objects".to_string()),
                data: None,
            };
        }
    };

    if array.is_empty() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("JSON array is empty".to_string()),
            data: None,
        };
    }

    // Collect all unique keys from all objects
    let mut headers: Vec<String> = Vec::new();
    for item in array {
        if let Some(obj) = item.as_object() {
            for key in obj.keys() {
                if !headers.contains(key) {
                    headers.push(key.clone());
                }
            }
        }
    }

    let mut csv_output = headers.join(",") + "\n";

    for item in array {
        if let Some(obj) = item.as_object() {
            let row: Vec<String> = headers
                .iter()
                .map(|h| {
                    obj.get(h)
                        .map(|v| {
                            let s = match v {
                                Value::String(s) => s.clone(),
                                Value::Null => String::new(),
                                _ => v.to_string(),
                            };
                            // Escape CSV values
                            if s.contains(',') || s.contains('"') || s.contains('\n') {
                                format!("\"{}\"", s.replace('"', "\"\""))
                            } else {
                                s
                            }
                        })
                        .unwrap_or_default()
                })
                .collect();
            csv_output.push_str(&row.join(","));
            csv_output.push('\n');
        }
    }

    match fs::write(output_path, &csv_output) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write CSV file: {}", e)),
            data: None,
        },
    }
}

pub fn convert_csv_to_html(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read CSV file: {}", e)),
                data: None,
            };
        }
    };

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let headers: Vec<String> = match reader.headers() {
        Ok(h) => h.iter().map(|s| s.to_string()).collect(),
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read CSV headers: {}", e)),
                data: None,
            };
        }
    };

    let filename = std::path::Path::new(input_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Data");

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
            margin: 40px;
            background: #f5f5f5;
        }}
        h1 {{ color: #333; }}
        table {{
            border-collapse: collapse;
            width: 100%;
            background: white;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }}
        th, td {{
            border: 1px solid #ddd;
            padding: 12px;
            text-align: left;
        }}
        th {{
            background: #4f46e5;
            color: white;
        }}
        tr:nth-child(even) {{
            background: #f9f9f9;
        }}
        tr:hover {{
            background: #f0f0f0;
        }}
    </style>
</head>
<body>
    <h1>{}</h1>
    <table>
        <thead>
            <tr>
"#,
        filename, filename
    );

    for header in &headers {
        html.push_str(&format!("                <th>{}</th>\n", escape_html(header)));
    }

    html.push_str("            </tr>\n        </thead>\n        <tbody>\n");

    for result in reader.records() {
        if let Ok(record) = result {
            html.push_str("            <tr>\n");
            for field in record.iter() {
                html.push_str(&format!("                <td>{}</td>\n", escape_html(field)));
            }
            html.push_str("            </tr>\n");
        }
    }

    html.push_str(
        r#"        </tbody>
    </table>
</body>
</html>"#,
    );

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

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
