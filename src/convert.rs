use image::GenericImageView;
use crate::types::{AsciiCell, ASCII_CHARS};

/// Converts an image into a 2D grid of ASCII cells
pub fn generate_ascii(img: &image::DynamicImage) -> Vec<Vec<AsciiCell>> {
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
