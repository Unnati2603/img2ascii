use image::GenericImageView;
// GenericImageView trait gives .dimensions() and .get_pixel() methods

use clap::Parser;
// Enables Args::parse()
// auto gen CLI parsing code

use clap::ValueEnum;
use clap::CommandFactory;

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Txt,
    Html,
    Ansi,
}
// Defines what values --output can accept


#[derive(Clone, Copy, Debug)]
struct AsciiCell {
    ch: char,
    r: u8,
    g: u8,
    b: u8,
}
// STRUCTURED OUTPUT


const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";

// for smoother
// const ASCII_CHARS: &[u8] =  b"$@B%8&WM#*oahkbdpqwmZ0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`. ";
//take mono space font into account
// const ASCII_CHARS: &[u8] =  b"$@B%8&WM#*/\\|()1{}[]?-_+~<>i!lI;:,\"^`. ";

use image::ImageError;
use std::io::ErrorKind;

/// Command-line arguments for img2ascii, the triple lines come in --help cmd
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

// NOT just a struct
// clap will generate code to parse CLI args into this struct
//command-line arguments

// Rust doesn’t execute #[derive(Parser)].
// The compiler reads it and generates code before your program runs.


/* TL DR;
What Rust is actually doing
The compiler + clap generate code at compile time
That code:
Reads command-line arguments
Parses them
Validates them
Fills Args struct

So after let args = Args::parse();
we have an Args struct filled with values from the command line
*/  



// Helper function to derive output filename based on input image path and desired format
// Takes the image path and output format, returns a string like "image.txt" or "image.html"
use std::path::Path;

fn output_filename(image_path: &str, format: &OutputFormat) -> String {
    // Extract the filename stem (name without extension) from the image path
    // If extraction fails, default to "output"
    let stem = Path::new(image_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    // Determine the file extension based on the output format
    let ext = match format {
        OutputFormat::Txt => "txt",
        OutputFormat::Html => "html",
        OutputFormat::Ansi => "ansi",
    };

    // Combine stem and extension into final filename
    format!("{}.{}", stem, ext)
}


// REPLACED WITH  render_html / render_ansi
// Function to handle output routing
// Takes the generated ASCII string, original image, and command-line args
// Returns Result,handle file I/O errors
// fn write_output(
//     ascii: &str,
//     img: &image::DynamicImage,
//     args: &Args,
// ) -> std::io::Result<()> {
//     match &args.output {
//         None => {
//             // No output format specified (-o flag not used) → print ASCII to terminal
//             print!("{}", ascii);
//             Ok(())
//         }

//         Some(format) => {
//             // Output format specified → save to file with appropriate extension
//             let filename = output_filename(&args.image, format);

//             match format {
//                 // For text and ANSI formats, write ASCII string directly to file
//                 OutputFormat::Txt => {
//                     let clean = strip_ansi(ascii);
//                     std::fs::write(&filename, clean)?;
//                 }              
//                 OutputFormat::Ansi => {
//                     std::fs::write(&filename, ascii)?;
//                 }
//                 // For HTML format, use specialized HTML writer function
//                 // OutputFormat::Html => {
//                 //     write_html(&filename, img)?;
//                 // }
//                 OutputFormat::Html => {
//                     let clean = if args.color {
//                         ascii.to_string()
//                     } else {
//                         strip_ansi(ascii)
//                     };
//                     write_html(&filename, &clean)?;
//                 }
//             }

//             // Print success message to stderr (doesn't interfere with ASCII output)
//             eprintln!("Saved ASCII output to {}", filename);
//             Ok(())
//         }
//     }
// }




/* OK ISSUE:
strip_ansi, 
when    --color --output txt
ASCII string contains ANSI escape sequences like \x1b[38;2;255;0;0m@\x1b[0m
Text files must not contain these — they’ll look like garbage.

What ANSI codes look like:  ESC [ ... m
    ESC = \x1b
    [ starts control sequence
    ends with a letter, usually m
So see \x1b, skip everything until you hit a letter.

so below is a regx free linear time func, handles ansii and color
*/

// fn strip_ansi(input: &str) -> String {
//     let mut out = String::with_capacity(input.len());
//     let mut chars = input.chars().peekable();

//     while let Some(c) = chars.next() {
//         if c == '\x1b' {
//             // Skip '['
//             if chars.peek() == Some(&'[') {
//                 chars.next();

//                 // Skip until we hit an alphabetic character (ANSI terminator)
//                 while let Some(&next) = chars.peek() {
//                     chars.next();
//                     if next.is_ascii_alphabetic() {
//                         break;
//                     }
//                 }
//             }
//         } else {
//             out.push(c);
//         }
//     }

//     out
// }





// html func
/*
ANSI color ≠ HTML color
They are different worlds, with different rules and representations.

Why ANSI color works in terminal (but nowhere else)
When you do this in Rust: \x1b[38;2;R;G;Bm@\x1b[0m
Its: Hey terminal emulator, please change the text color before printing @

ANSI is:
    Stateful
    Implicit
    Side-effect based
    Only meaningful to terminals
TXT files, browsers, HTML parsers do not execute control codes.

HTML cannot “just use” ANSI. 
HTML rules: No hidden state; No “reset color”; Every visual effect must be declared per element

So \x1b[38;2;255;0;0m@\x1b[0m MEANS NOTHING TO browsers

TLDR: 
Current: Pixel → ASCII char → ANSI escape codes → String
REq: Pixel → ASCII char + RGB → <span style="color:rgb(...)">

HOw? 
i have (r, g, b, ascii_char) for each pixel.  So instead of generating strings early, generate semantic data.

!!!!!!!!!! split “generation” from “rendering”!!!!!!!!!!

What I am doing: generate_ascii() -> String (with ANSI inside)
WHAT TO DO: 
struct AsciiPixel {
    ch: char,
    r: u8,
    g: u8,
    b: u8,
}
then
    Vec<Vec<AsciiPixel>>

this allows:
    Terminal ANSI
    HTML with <span>
    Plain text
    GIF
    Video frames
ALL FROM THE SAME data


*/

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

// OLD GENERATE THAT GAVE sTRING , PROBLEM WITH HTML
// fn generate_ascii(img: &image::DynamicImage, args: &Args) -> String {
//      let mut out = String::new(); 
//     // LOOP THROUGH ALL PIXELS IN THE RESIZED IMAGE
//     for y in 0..img.height(){
//         for x in 0..img.width(){
//             let rgb_pixel = img.get_pixel(x, y);
//             let [r, g, b, _] = rgb_pixel.0;  // Extract R, G, B values 
            
//             // // CALCULATE BRIGHTNESS using standard luminosity formula
//             // (Red is weighted less because human eyes are less sensitive to red)
//             let brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
            
//             // MAP BRIGHTNESS TO ASCII character
//             let unnati = (brightness as usize * ASCII_CHARS.len().saturating_sub(1)) / 256;
//             let char_to_print = ASCII_CHARS[unnati] as char;
            
//             // OUTPUT WITH OR WITHOUT COLOR
//             if args.color {
//                 // If -c flag is present: use ANSI 24-bit true color codes
//                 // \x1b[38;2;R;G;Bm = foreground color to RGB
//                 // \x1b[0m = reset color back to default
                
//                 // print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, char_to_print);

//                  out.push_str(&format!(
//                     "\x1b[38;2;{};{};{}m{}\x1b[0m",
//                     r, g, b, char_to_print
//                 ));

//             } else {
//                 // Default character without color
                
//                 // print!("{}", char_to_print);

//                 out.push(char_to_print);
//             }
//         }
//         // println!(); 
//          out.push('\n');
//     }
// out
// }

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
    // img.resize_exact(width, height, filter)
    // resize_exact forces the exact dimensions (ignores original aspect ratio)
    // we handle aspect ratio ourselves with char_aspect correction
    /*
    Nearest Neighbor resizing
    For every pixel in the new image:
    1. Find the closest pixel in the old image
    2. Copy its value directly
    3. No blending, no smoothing
    */

//   let filename = output_filename(&args.image, &OutputFormat::Html);
//     std::fs::write(filename, html)?;
  
    let ascii_cells = generate_ascii(&resized_img);

    // match args.output {
    //     Some(OutputFormat::Html) => {
    //         let html = render_html(&ascii_cells);
    //         let filename = output_filename(&args.image, &OutputFormat::Html);
    //         std::fs::write(filename, html)?;
    //     }

    //     Some(OutputFormat::Ansi) => {
    //         let text = render_ansi(&ascii_cells, args.color);
    //         let filename = output_filename(&args.image, &OutputFormat::Ansi);
    //         std::fs::write(filename, text)?;
    //     }

    //     Some(OutputFormat::Txt) => {
    //         let text = render_ansi(&ascii_cells, false); // no ANSI
    //         let filename = output_filename(&args.image, &OutputFormat::Txt);
    //         std::fs::write(filename, text)?;
    //     }

    //     None => {
    //         let text = render_ansi(&ascii_cells, args.color);
    //         print!("{}", text);
    //     }
    // }

    // Always render terminal output
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




// if let Err(e) = write_output(&ascii, &resized_img, &args) {
//     eprintln!("Failed to write output: {}", e);
//     std::process::exit(6);
// }

Ok(())

}
