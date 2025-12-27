use image::GenericImageView;
use clap::Parser;
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Txt,
    Html,
    Ansi,
}

#[derive(Clone, Copy, Debug)]
struct AsciiCell {
    ch: char,
    r: u8,
    g: u8,
    b: u8,
}

const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";

use image::ImageError;
use std::io::ErrorKind;

/// Command-line arguments for img2ascii
#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    /// Path to the input image file
    #[arg(value_name = "IMAGE", index = 1)]
    image: String,

    /// Output width in characters
    #[arg(short, long, default_value_t = 80)]
    width: u32,

    /// Output height in characters (overrides aspect ratio)
    #[arg(short = 'H', long)]
    height: Option<u32>,

    /// Enable colored ASCII output
    #[arg(short, long)]
    color: bool,

    /// Output format (txt, html, ansi). If omitted, prints to terminal.
    #[arg(short = 'o', long = "output", value_enum)]
    output: Option<OutputFormat>,
}  



// Helper function to derive output filename based on input image path and desired format
use std::path::Path;

fn output_filename(image_path: &str, format: &OutputFormat) -> String {
    let stem = Path::new(image_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let ext = match format {
        OutputFormat::Txt => "txt",
        OutputFormat::Html => "html",
        OutputFormat::Ansi => "ansi",
    };

    // Combine stem and extension into final filename
    format!("{}.{}", stem, ext)
}



fn render_html(cells: &[Vec<AsciiCell>]) -> String {
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





fn generate_ascii(img: &image::DynamicImage) -> Vec<Vec<AsciiCell>> {
    let mut rows = Vec::with_capacity(img.height() as usize);

    for y in 0..img.height() {
        let mut row = Vec::with_capacity(img.width() as usize);

        for x in 0..img.width() {
            let [r, g, b, _] = img.get_pixel(x, y).0;

            // brightness (luminosity)
            let brightness =
                (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;

            let idx = (brightness as usize * ASCII_CHARS.len()) / 256;
            let idx = idx.min(ASCII_CHARS.len() - 1);

            row.push(AsciiCell {
                ch: ASCII_CHARS[idx] as char,
                r,
                g,
                b,
            });
        }

        rows.push(row);
    }

    rows
}

fn render_ansi(cells: &[Vec<AsciiCell>], color: bool) -> String {
    let mut out = String::new();

    for row in cells {
        for cell in row {
            if color {
                out.push_str(&format!(
                    "\x1b[38;2;{};{};{}m{}\x1b[0m",
                    cell.r, cell.g, cell.b, cell.ch
                ));
            } else {
                out.push(cell.ch);
            }
        }
        out.push('\n');
    }

    out
}


// MAIN FUNCTION BOMBOMBOOOOOOO

// fn main() {
fn main() -> std::io::Result<()> {

    // Parse command-line arguments into Args struct using clap
    let args = Args::parse();

    // Try to open the image file, handle errors for file not found + unsupported format
    // don’t define Unsupported anywhere.
    // It’s defined by the image crate and returned by image::open() when it can’t decode the file.
    let img = match image::open(&args.image) {
        Ok(img) => img,
        Err(ImageError::IoError(ref e)) if e.kind() == ErrorKind::NotFound => {
        eprintln!("Error: File not found: {}", args.image);
        std::process::exit(2);
    }
        Err(ImageError::Unsupported(_)) => {
        eprintln!("Error: Unsupported image format: {}", args.image);
        std::process::exit(3);
    }
        Err(e) => {
        eprintln!("Failed to open image '{}': {}", args.image, e);
        std::process::exit(1);
    }
    };

    // Get image dimensions (width, height)
    let (w, h) = img.dimensions();
    // Check for zero width or height
    if w == 0 || h == 0 {
        eprintln!("Error: Image width or height is zero ({}x{})", w, h);
        std::process::exit(4);
    }

    // Convert dimensions to f32 for aspect ratio calculation
    let w = w as f32;
    let h = h as f32;
    // Calculate aspect ratio (height/width)
    let aspect_ratio = h as f32 / w as f32;

    // Desired output width in characters (from CLI)
    let new_w = args.width;

    // Correction factor for character aspect ratio (terminal characters are taller than wide)
    let char_aspect = 0.43;
    // Calculate output height in characters, correcting for char aspect
    let mut new_h = (new_w as f32 * aspect_ratio * char_aspect) as u32;

    // If user provided a custom height, override calculated height
    if let Some(h) = args.height {
        new_h = h;
    }
    use image::imageops::FilterType;
    // Bring the FilterType enum into scope

    // Resize the image to the desired character dimensions using nearest neighbor
    let resized_img = img.resize_exact(
        new_w, new_h, FilterType::Nearest,
    );
    let ascii_cells = generate_ascii(&resized_img);

let terminal_text = render_ansi(&ascii_cells, args.color);

// 1. Print to terminal (ALWAYS)
print!("{}", terminal_text);
// eprintln!("DEBUG → color flag = {}", args.color);

// 2. Optionally save to file
if let Some(format) = &args.output {
    let filename = output_filename(&args.image, format);

    match format {
        OutputFormat::Html => {
            let html = render_html(&ascii_cells);
            std::fs::write(&filename, html)?;
        }

        OutputFormat::Ansi => {
            std::fs::write(&filename, terminal_text)?;
        }

        OutputFormat::Txt => {
            let plain = render_ansi(&ascii_cells, false);
            std::fs::write(&filename, plain)?;
        }
    }

    eprintln!("Saved output to {}", filename);
}

Ok(())

}
