use std::fs;
use std::io::BufReader;
use calamine::{Reader, Xlsx, Xls, Ods, Data};
use crate::ConversionResult;
use serde_json::{json, Value};

/// Convert Excel XLSX file to CSV
pub fn convert_xlsx_to_csv(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::File::open(input_path) {
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

    let mut workbook: Xlsx<_> = match Xlsx::new(BufReader::new(file)) {
        Ok(wb) => wb,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read XLSX file: {}", e)),
                data: None,
            };
        }
    };

    let sheets = workbook.sheet_names().to_vec();
    if sheets.is_empty() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("No sheets found in workbook".to_string()),
            data: None,
        };
    }

    let range = match workbook.worksheet_range(&sheets[0]) {
        Ok(r) => r,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read sheet: {}", e)),
                data: None,
            };
        }
    };

    let mut csv_output = String::new();
    for row in range.rows() {
        let row_str: Vec<String> = row.iter().map(|cell| {
            let value = cell_to_string(cell);
            if value.contains(',') || value.contains('"') || value.contains('\n') {
                format!("\"{}\"", value.replace('"', "\"\""))
            } else {
                value
            }
        }).collect();
        csv_output.push_str(&row_str.join(","));
        csv_output.push('\n');
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

/// Convert Excel XLS file to CSV
pub fn convert_xls_to_csv(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::File::open(input_path) {
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

    let mut workbook: Xls<_> = match Xls::new(BufReader::new(file)) {
        Ok(wb) => wb,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read XLS file: {}", e)),
                data: None,
            };
        }
    };

    let sheets = workbook.sheet_names().to_vec();
    if sheets.is_empty() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("No sheets found in workbook".to_string()),
            data: None,
        };
    }

    let range = match workbook.worksheet_range(&sheets[0]) {
        Ok(r) => r,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read sheet: {}", e)),
                data: None,
            };
        }
    };

    let mut csv_output = String::new();
    for row in range.rows() {
        let row_str: Vec<String> = row.iter().map(|cell| {
            let value = cell_to_string(cell);
            if value.contains(',') || value.contains('"') || value.contains('\n') {
                format!("\"{}\"", value.replace('"', "\"\""))
            } else {
                value
            }
        }).collect();
        csv_output.push_str(&row_str.join(","));
        csv_output.push('\n');
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

/// Convert ODS file to CSV
pub fn convert_ods_to_csv(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::File::open(input_path) {
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

    let mut workbook: Ods<_> = match Ods::new(BufReader::new(file)) {
        Ok(wb) => wb,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read ODS file: {}", e)),
                data: None,
            };
        }
    };

    let sheets = workbook.sheet_names().to_vec();
    if sheets.is_empty() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("No sheets found in workbook".to_string()),
            data: None,
        };
    }

    let range = match workbook.worksheet_range(&sheets[0]) {
        Ok(r) => r,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read sheet: {}", e)),
                data: None,
            };
        }
    };

    let mut csv_output = String::new();
    for row in range.rows() {
        let row_str: Vec<String> = row.iter().map(|cell| {
            let value = cell_to_string(cell);
            if value.contains(',') || value.contains('"') || value.contains('\n') {
                format!("\"{}\"", value.replace('"', "\"\""))
            } else {
                value
            }
        }).collect();
        csv_output.push_str(&row_str.join(","));
        csv_output.push('\n');
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

/// Convert XLSX to JSON
pub fn convert_xlsx_to_json(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::File::open(input_path) {
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

    let mut workbook: Xlsx<_> = match Xlsx::new(BufReader::new(file)) {
        Ok(wb) => wb,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read XLSX file: {}", e)),
                data: None,
            };
        }
    };

    workbook_to_json(&mut workbook, output_path)
}

/// Convert XLS to JSON
pub fn convert_xls_to_json(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::File::open(input_path) {
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

    let mut workbook: Xls<_> = match Xls::new(BufReader::new(file)) {
        Ok(wb) => wb,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read XLS file: {}", e)),
                data: None,
            };
        }
    };

    workbook_to_json(&mut workbook, output_path)
}

/// Convert ODS to JSON
pub fn convert_ods_to_json(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::File::open(input_path) {
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

    let mut workbook: Ods<_> = match Ods::new(BufReader::new(file)) {
        Ok(wb) => wb,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read ODS file: {}", e)),
                data: None,
            };
        }
    };

    workbook_to_json(&mut workbook, output_path)
}

fn workbook_to_json<R: Reader<BufReader<fs::File>>>(workbook: &mut R, output_path: &str) -> ConversionResult 
where <R as Reader<BufReader<fs::File>>>::Error: std::fmt::Display {
    let sheets = workbook.sheet_names().to_vec();
    
    if sheets.is_empty() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("No sheets found in workbook".to_string()),
            data: None,
        };
    }

    let range = match workbook.worksheet_range(&sheets[0]) {
        Ok(r) => r,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read sheet: {}", e)),
                data: None,
            };
        }
    };

    let rows: Vec<Vec<Data>> = range.rows().map(|r| r.to_vec()).collect();
    
    if rows.is_empty() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("Sheet is empty".to_string()),
            data: None,
        };
    }

    // First row is headers
    let headers: Vec<String> = rows[0].iter().map(|c| cell_to_string(c)).collect();
    
    let mut records: Vec<Value> = Vec::new();
    for row in rows.iter().skip(1) {
        let mut obj = serde_json::Map::new();
        for (i, cell) in row.iter().enumerate() {
            if i < headers.len() {
                obj.insert(headers[i].clone(), cell_to_json_value(cell));
            }
        }
        records.push(Value::Object(obj));
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

/// Convert XLSX to HTML table
pub fn convert_xlsx_to_html(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::File::open(input_path) {
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

    let mut workbook: Xlsx<_> = match Xlsx::new(BufReader::new(file)) {
        Ok(wb) => wb,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read XLSX file: {}", e)),
                data: None,
            };
        }
    };

    workbook_to_html(&mut workbook, input_path, output_path)
}

/// Convert XLS to HTML table
pub fn convert_xls_to_html(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::File::open(input_path) {
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

    let mut workbook: Xls<_> = match Xls::new(BufReader::new(file)) {
        Ok(wb) => wb,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read XLS file: {}", e)),
                data: None,
            };
        }
    };

    workbook_to_html(&mut workbook, input_path, output_path)
}

/// Convert ODS to HTML table
pub fn convert_ods_to_html(input_path: &str, output_path: &str) -> ConversionResult {
    let file = match fs::File::open(input_path) {
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

    let mut workbook: Ods<_> = match Ods::new(BufReader::new(file)) {
        Ok(wb) => wb,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read ODS file: {}", e)),
                data: None,
            };
        }
    };

    workbook_to_html(&mut workbook, input_path, output_path)
}

fn workbook_to_html<R: Reader<BufReader<fs::File>>>(workbook: &mut R, input_path: &str, output_path: &str) -> ConversionResult 
where <R as Reader<BufReader<fs::File>>>::Error: std::fmt::Display {
    let sheets = workbook.sheet_names().to_vec();
    
    if sheets.is_empty() {
        return ConversionResult {
            success: false,
            output_path: None,
            error: Some("No sheets found in workbook".to_string()),
            data: None,
        };
    }

    let range = match workbook.worksheet_range(&sheets[0]) {
        Ok(r) => r,
        Err(e) => {
            return ConversionResult {
                success: false,
                output_path: None,
                error: Some(format!("Failed to read sheet: {}", e)),
                data: None,
            };
        }
    };

    let filename = std::path::Path::new(input_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Spreadsheet");

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
"#,
        filename, filename
    );

    let rows: Vec<Vec<Data>> = range.rows().map(|r| r.to_vec()).collect();
    
    if !rows.is_empty() {
        // First row as header
        html.push_str("        <thead>\n            <tr>\n");
        for cell in &rows[0] {
            html.push_str(&format!("                <th>{}</th>\n", escape_html(&cell_to_string(cell))));
        }
        html.push_str("            </tr>\n        </thead>\n        <tbody>\n");

        // Data rows
        for row in rows.iter().skip(1) {
            html.push_str("            <tr>\n");
            for cell in row {
                html.push_str(&format!("                <td>{}</td>\n", escape_html(&cell_to_string(cell))));
            }
            html.push_str("            </tr>\n");
        }
        html.push_str("        </tbody>\n");
    }

    html.push_str("    </table>\n</body>\n</html>");

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

fn cell_to_string(cell: &Data) -> String {
    match cell {
        Data::Empty => String::new(),
        Data::String(s) => s.clone(),
        Data::Int(i) => i.to_string(),
        Data::Float(f) => f.to_string(),
        Data::Bool(b) => b.to_string(),
        Data::Error(e) => format!("#ERR:{:?}", e),
        Data::DateTime(dt) => format!("{}", dt),
        Data::DateTimeIso(s) => s.clone(),
        Data::DurationIso(s) => s.clone(),
    }
}

fn cell_to_json_value(cell: &Data) -> Value {
    match cell {
        Data::Empty => Value::Null,
        Data::String(s) => json!(s),
        Data::Int(i) => json!(i),
        Data::Float(f) => json!(f),
        Data::Bool(b) => json!(b),
        Data::Error(_) => Value::Null,
        Data::DateTime(dt) => json!(dt.to_string()),
        Data::DateTimeIso(s) => json!(s),
        Data::DurationIso(s) => json!(s),
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
