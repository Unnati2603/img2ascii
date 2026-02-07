//! Image to ASCII Conversion Module
//!
//! Handles the core conversion logic from image pixels to ASCII characters.
//! Converts each pixel to an ASCII character based on its brightness level,
//! preserving color information for colored output.

use image::GenericImageView;
use crate::types::{AsciiCell, ASCII_CHARS, EDGE_ASCII_CHARS, calculate_luminance};

/// Converts an image into a 2D grid of ASCII cells
pub fn generate_ascii(img: &image::DynamicImage) -> Vec<Vec<AsciiCell>> {
    generate_ascii_with_charset(img, ASCII_CHARS)
}

/// Converts edge-detected image to ASCII using directional edge characters
pub fn generate_ascii_edges(img: &image::DynamicImage) -> Vec<Vec<AsciiCell>> {
    let mut rows = Vec::with_capacity(img.height() as usize);

    for y in 0..img.height() {
        let mut row = Vec::with_capacity(img.width() as usize);

        for x in 0..img.width() {
            let [r, g, b, dir_encoded] = img.get_pixel(x, y).0;

            // Check if it's an edge (non-black)
            if r == 0 && g == 0 && b == 0 {
                // No edge - use space
                row.push(AsciiCell {
                    ch: ' ',
                    r, g, b,
                });
            } else {
                // Edge detected - use direction to pick character
                // dir_encoded contains the character index
                let idx = (dir_encoded as usize).min(EDGE_ASCII_CHARS.len() - 1);
                
                row.push(AsciiCell {
                    ch: EDGE_ASCII_CHARS[idx] as char,
                    r, g, b,
                });
            }
        }

        rows.push(row);
    }

    rows
}

/// Internal function to generate ASCII with a specific character set
fn generate_ascii_with_charset(img: &image::DynamicImage, charset: &[u8]) -> Vec<Vec<AsciiCell>> {
    let mut rows = Vec::with_capacity(img.height() as usize);

    for y in 0..img.height() {
        let mut row = Vec::with_capacity(img.width() as usize);

        for x in 0..img.width() {
            let [r, g, b, _] = img.get_pixel(x, y).0;

            // brightness (luminosity)
            let brightness = calculate_luminance(r, g, b) as u8;

            let idx = (brightness as usize * charset.len()) / 256;
            let idx = idx.min(charset.len() - 1);

            row.push(AsciiCell {
                ch: charset[idx] as char,
                r,
                g,
                b,
            });
        }

        rows.push(row);
    }

    rows
}
