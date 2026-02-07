//! Type Definitions Module
//!
//! Contains core data structures and constants used throughout the application,
//! including the ASCII character set and the AsciiCell structure for storing
//! character and color information.

// ASCII characters ordered from darkest to lightest
pub const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";

// Edge ASCII characters for edge detection mode (dense to sparse)
pub const EDGE_ASCII_CHARS: &[u8] = b"|/-\\+*. ";

// Structure to hold ASCII character and its RGB color
#[derive(Clone, Copy, Debug)]
pub struct AsciiCell {
    pub ch: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Calculate luminance (brightness) from RGB values using standard formula
///
/// Uses the ITU-R BT.601 luma coefficients which match human perception:
/// - Red: 29.9%
/// - Green: 58.7% (human eye is most sensitive to green)
/// - Blue: 11.4%
///
/// # Arguments
/// * `r` - Red channel value (0-255)
/// * `g` - Green channel value (0-255)
/// * `b` - Blue channel value (0-255)
///
/// # Returns
/// Luminance value as f32 (0.0-255.0)
#[inline]
pub fn calculate_luminance(r: u8, g: u8, b: u8) -> f32 {
    0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32
}
