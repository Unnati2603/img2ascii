//! HTML Rendering Module
//!
//! Renders ASCII art as a styled HTML document with colored characters.
//! Generates a complete HTML page with inline CSS for proper display.

use crate::types::AsciiCell;

/// Renders ASCII cells as an HTML document with inline styles
pub fn render_html(cells: &[Vec<AsciiCell>]) -> String {
    let mut html = String::new();

    html.push_str(r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<style>
pre {
  font-family: monospace;
  line-height: 1;
  font-size: 8px;
}
</style>
</head>
<body>
<pre>
"#);

    for row in cells {
        for cell in row {
            html.push_str(&format!(
                r#"<span style="color: rgb({},{},{})">{}</span>"#,
                cell.r,
                cell.g,
                cell.b,
                html_escape(cell.ch)
            ));
        }
        html.push('\n');
    }
    html.push_str("</pre></body></html>");
    html

}

fn html_escape(c: char) -> String {
    match c {
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '&' => "&amp;".to_string(),
        _ => c.to_string(),
    }
}
