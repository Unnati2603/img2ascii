//! Core Application Logic Module
//!
//! Contains the main run() function that orchestrates the entire image-to-ASCII
//! conversion process: parsing arguments, loading the image, resizing it,
//! converting to ASCII, and outputting in the requested format.

use clap::Parser;
use image::{GenericImageView, ImageError};
use std::io::ErrorKind;
use image::imageops::FilterType;

use crate::cli::{Args, OutputFormat};
use crate::output::output_filename;
use crate::convert::{generate_ascii, generate_ascii_edges};
use crate::renderansi::render_ansi;
use crate::renderhtml::render_html;
use crate::edge::sobel_edge_detection;

// Main logic function
pub fn run() -> std::io::Result<()> {
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

    let aspect_ratio = h as f32 / w as f32;

    let new_w = args.width;

    // Correction factor for character aspect ratio (terminal characters are taller than wide)
    let char_aspect = 0.43;
    // Calculate output height in characters, correcting for char aspect
    let mut new_h = (new_w as f32 * aspect_ratio * char_aspect) as u32;

    // If user provided a custom height, override calculated height
    if let Some(h) = args.height {
        new_h = h;
    }

    // Resize the image to the desired character dimensions using nearest neighbor
    let resized_img = img.resize_exact(
        new_w, new_h, FilterType::Nearest,
    );
    
    // Apply edge detection AFTER resize if requested
    let processed_img = if args.edges {
        sobel_edge_detection(&resized_img, args.edge_threshold)
    } else {
        resized_img
    };
    
    // Convert to ASCII using appropriate character set
    let ascii_cells = if args.edges {
        generate_ascii_edges(&processed_img)
    } else {
        generate_ascii(&processed_img)
    };

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
