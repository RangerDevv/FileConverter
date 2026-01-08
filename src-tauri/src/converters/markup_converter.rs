use std::fs;
use crate::ConversionResult;
use serde_json::{json, Value};
use pulldown_cmark::{Parser, Options, html};

/// Convert XML to JSON
pub fn convert_xml_to_json(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read XML file: {}", e)),
                data: None,
            };
        }
    };

    let json_value = match xml_to_json(&content) {
        Ok(v) => v,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse XML: {}", e)),
                data: None,
            };
        }
    };

    let json_output = match serde_json::to_string_pretty(&json_value) {
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

fn xml_to_json(xml: &str) -> Result<Value, String> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<(String, Value)> = vec![("root".to_string(), json!({}))];

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut obj = serde_json::Map::new();
                
                // Add attributes
                for attr in e.attributes().flatten() {
                    let key = format!("@{}", String::from_utf8_lossy(attr.key.as_ref()));
                    let value = String::from_utf8_lossy(&attr.value).to_string();
                    obj.insert(key, json!(value));
                }
                
                stack.push((name, Value::Object(obj)));
            }
            Ok(Event::End(_)) => {
                if stack.len() > 1 {
                    let (name, value) = stack.pop().unwrap();
                    if let Some((_, parent)) = stack.last_mut() {
                        if let Value::Object(parent_obj) = parent {
                            if parent_obj.contains_key(&name) {
                                // Convert to array if key already exists
                                let existing = parent_obj.remove(&name).unwrap();
                                if let Value::Array(mut arr) = existing {
                                    arr.push(value);
                                    parent_obj.insert(name, Value::Array(arr));
                                } else {
                                    parent_obj.insert(name, json!([existing, value]));
                                }
                            } else {
                                parent_obj.insert(name, value);
                            }
                        }
                    }
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().map_err(|e| e.to_string())?.trim().to_string();
                if !text.is_empty() {
                    if let Some((_, value)) = stack.last_mut() {
                        if let Value::Object(obj) = value {
                            if obj.is_empty() {
                                *value = json!(text);
                            } else {
                                obj.insert("#text".to_string(), json!(text));
                            }
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e)),
            _ => {}
        }
    }

    if let Some((_, root)) = stack.pop() {
        Ok(root)
    } else {
        Ok(json!({}))
    }
}

/// Convert JSON to XML
pub fn convert_json_to_xml(input_path: &str, output_path: &str) -> ConversionResult {
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

    let json_value: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse JSON: {}", e)),
                data: None,
            };
        }
    };

    let xml_output = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<root>\n{}</root>",
        json_to_xml(&json_value, 1)
    );

    match fs::write(output_path, &xml_output) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write XML file: {}", e)),
            data: None,
        },
    }
}

fn json_to_xml(value: &Value, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    match value {
        Value::Null => String::new(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => escape_xml(s),
        Value::Array(arr) => {
            arr.iter()
                .map(|v| format!("{}<item>\n{}{}</item>\n", indent_str, json_to_xml(v, indent + 1), indent_str))
                .collect()
        }
        Value::Object(obj) => {
            let mut result = String::new();
            for (key, val) in obj {
                let safe_key = key.replace(|c: char| !c.is_alphanumeric() && c != '_', "_");
                match val {
                    Value::Array(arr) => {
                        for item in arr {
                            result.push_str(&format!(
                                "{}<{}>\n{}{}</{}>\n",
                                indent_str,
                                safe_key,
                                json_to_xml(item, indent + 1),
                                indent_str,
                                safe_key
                            ));
                        }
                    }
                    _ => {
                        let inner = json_to_xml(val, indent + 1);
                        if inner.contains('\n') {
                            result.push_str(&format!(
                                "{}<{}>\n{}{}</{}>\n",
                                indent_str, safe_key, inner, indent_str, safe_key
                            ));
                        } else {
                            result.push_str(&format!(
                                "{}<{}>{}</{}>\n",
                                indent_str, safe_key, inner, safe_key
                            ));
                        }
                    }
                }
            }
            result
        }
    }
}

/// Convert YAML to JSON
pub fn convert_yaml_to_json(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read YAML file: {}", e)),
                data: None,
            };
        }
    };

    let yaml_value: Value = match serde_yaml::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse YAML: {}", e)),
                data: None,
            };
        }
    };

    let json_output = match serde_json::to_string_pretty(&yaml_value) {
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

/// Convert JSON to YAML
pub fn convert_json_to_yaml(input_path: &str, output_path: &str) -> ConversionResult {
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

    let json_value: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse JSON: {}", e)),
                data: None,
            };
        }
    };

    let yaml_output = match serde_yaml::to_string(&json_value) {
        Ok(y) => y,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to create YAML: {}", e)),
                data: None,
            };
        }
    };

    match fs::write(output_path, &yaml_output) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write YAML file: {}", e)),
            data: None,
        },
    }
}

/// Convert TOML to JSON
pub fn convert_toml_to_json(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read TOML file: {}", e)),
                data: None,
            };
        }
    };

    let toml_value: toml::Value = match toml::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse TOML: {}", e)),
                data: None,
            };
        }
    };

    // Convert TOML value to JSON value
    let json_value = toml_to_json_value(&toml_value);

    let json_output = match serde_json::to_string_pretty(&json_value) {
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

fn toml_to_json_value(toml_val: &toml::Value) -> Value {
    match toml_val {
        toml::Value::String(s) => json!(s),
        toml::Value::Integer(i) => json!(i),
        toml::Value::Float(f) => json!(f),
        toml::Value::Boolean(b) => json!(b),
        toml::Value::Datetime(dt) => json!(dt.to_string()),
        toml::Value::Array(arr) => {
            Value::Array(arr.iter().map(toml_to_json_value).collect())
        }
        toml::Value::Table(table) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in table {
                obj.insert(k.clone(), toml_to_json_value(v));
            }
            Value::Object(obj)
        }
    }
}

/// Convert JSON to TOML
pub fn convert_json_to_toml(input_path: &str, output_path: &str) -> ConversionResult {
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

    let json_value: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to parse JSON: {}", e)),
                data: None,
            };
        }
    };

    let toml_value = json_to_toml_value(&json_value);

    let toml_output = match toml::to_string_pretty(&toml_value) {
        Ok(t) => t,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to create TOML: {}", e)),
                data: None,
            };
        }
    };

    match fs::write(output_path, &toml_output) {
        Ok(_) => ConversionResult {
            success: true,
            output_path: Some(output_path.to_string()),
            error: None,
            data: None,
        },
        Err(e) => ConversionResult {
            success: false,
            output_path: None,
            error: Some(format!("Failed to write TOML file: {}", e)),
            data: None,
        },
    }
}

fn json_to_toml_value(json_val: &Value) -> toml::Value {
    match json_val {
        Value::Null => toml::Value::String("null".to_string()),
        Value::Bool(b) => toml::Value::Boolean(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                toml::Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                toml::Value::Float(f)
            } else {
                toml::Value::String(n.to_string())
            }
        }
        Value::String(s) => toml::Value::String(s.clone()),
        Value::Array(arr) => {
            toml::Value::Array(arr.iter().map(json_to_toml_value).collect())
        }
        Value::Object(obj) => {
            let mut table = toml::map::Map::new();
            for (k, v) in obj {
                table.insert(k.clone(), json_to_toml_value(v));
            }
            toml::Value::Table(table)
        }
    }
}

/// Convert Markdown to HTML
pub fn convert_markdown_to_html(input_path: &str, output_path: &str) -> ConversionResult {
    let content = match fs::read_to_string(input_path) {
        Ok(c) => c,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read Markdown file: {}", e)),
                data: None,
            };
        }
    };

    let filename = std::path::Path::new(input_path)
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("Document");

    // Set up parser options
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content, options);

    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    let html_output = format!(
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
        }}
        h1, h2, h3, h4, h5, h6 {{
            color: #222;
            margin-top: 24px;
            margin-bottom: 16px;
        }}
        code {{
            background: #f4f4f4;
            padding: 2px 6px;
            border-radius: 3px;
            font-family: 'SF Mono', Consolas, monospace;
        }}
        pre {{
            background: #f4f4f4;
            padding: 16px;
            border-radius: 6px;
            overflow-x: auto;
        }}
        pre code {{
            background: none;
            padding: 0;
        }}
        blockquote {{
            border-left: 4px solid #ddd;
            margin: 0;
            padding-left: 16px;
            color: #666;
        }}
        table {{
            border-collapse: collapse;
            width: 100%;
            margin: 16px 0;
        }}
        th, td {{
            border: 1px solid #ddd;
            padding: 8px 12px;
            text-align: left;
        }}
        th {{
            background: #f5f5f5;
        }}
        img {{
            max-width: 100%;
            height: auto;
        }}
        a {{
            color: #4f46e5;
        }}
        ul, ol {{
            padding-left: 24px;
        }}
        li {{
            margin: 4px 0;
        }}
        input[type="checkbox"] {{
            margin-right: 8px;
        }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        escape_xml(filename),
        html_content
    );

    match fs::write(output_path, &html_output) {
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

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
