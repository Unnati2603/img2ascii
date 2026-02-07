//! Edge Detection Module
//!
//! Implements Sobel edge detection for preprocessing images before ASCII conversion.
//! Uses both edge magnitude AND direction to map to appropriate ASCII characters.
//!
//! ## How Sobel Works
//!
//! 1. Apply two 3×3 convolution kernels (Gx for horizontal, Gy for vertical)
//! 2. Calculate gradient magnitude: sqrt(Gx² + Gy²)
//! 3. Calculate gradient direction: atan2(Gy, Gx)
//! 4. Map direction to characters: | for vertical, - for horizontal, /\ for diagonals
//! 5. Use magnitude to modulate brightness

use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use std::f32::consts::PI;

/// Sobel horizontal gradient kernel (Gx) - detects vertical edges
const SOBEL_GX: [[i32; 3]; 3] = [
    [-1, 0, 1],
    [-2, 0, 2],
    [-1, 0, 1],
];

/// Sobel vertical gradient kernel (Gy) - detects horizontal edges
const SOBEL_GY: [[i32; 3]; 3] = [
    [-1, -2, -1],
    [ 0,  0,  0],
    [ 1,  2,  1],
];

/// Apply Sobel edge detection with directional encoding
///
/// # Arguments
/// * `img` - Input image
/// * `threshold` - Minimum edge strength to keep (0-255)
///
/// # Returns
/// RGBA image where RGB encodes edge direction and alpha encodes magnitude
pub fn sobel_edge_detection(img: &DynamicImage, threshold: u8) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = RgbaImage::new(width, height);
    
    // Process interior pixels (skip 1-pixel border)
    for y in 1..height-1 {
        for x in 1..width-1 {
            // Apply Sobel kernels on luminance
            let gx = convolve_3x3_luminance(img, x, y, &SOBEL_GX);
            let gy = convolve_3x3_luminance(img, x, y, &SOBEL_GY);
            
            // Calculate edge magnitude using Euclidean norm
            let magnitude = ((gx * gx + gy * gy) as f32).sqrt();
            let magnitude = magnitude.min(255.0) as u8;
            
            if magnitude >= threshold {
                // Calculate edge direction
                let angle = (gy as f32).atan2(gx as f32);
                
                // Get original color
                let [r, g, b, _] = img.get_pixel(x, y).0;
                
                // Encode direction in RGB for ASCII mapping
                // We'll decode this in convert.rs to pick the right character
                let dir_encoded = encode_direction(angle);
                
                // Use original color but encode direction info in alpha channel for now
                // Actually, let's use a simple encoding: brightness = magnitude
                output.put_pixel(x, y, Rgba([r, g, b, dir_encoded]));
            } else {
                // No edge - black
                output.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            }
        }
    }
    
    // Borders remain black
    for y in 0..height {
        for x in 0..width {
            if x == 0 || y == 0 || x == width-1 || y == height-1 {
                output.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            }
        }
    }
    
    DynamicImage::ImageRgba8(output)
}

/// Encode edge direction as a value 0-255
/// Divides 360 degrees into 8 sectors for 8 possible edge characters
fn encode_direction(angle: f32) -> u8 {
    // Normalize angle to 0-2π
    let normalized = if angle < 0.0 { angle + 2.0 * PI } else { angle };
    
    // Divide into 8 sectors (45 degrees each)
    // 0: → (horizontal right)
    // 1: ↗ (diagonal up-right) 
    // 2: ↑ (vertical up)
    // 3: ↖ (diagonal up-left)
    // 4: ← (horizontal left)
    // 5: ↙ (diagonal down-left)
    // 6: ↓ (vertical down)
    // 7: ↘ (diagonal down-right)
    
    let sector = ((normalized / (PI / 4.0) + 0.5) as u8) % 8;
    
    // Map to character indices in EDGE_ASCII_CHARS = "|/-\\+*. "
    match sector {
        0 | 4 => 2, // - horizontal
        1 | 5 => 1, // / diagonal
        2 | 6 => 0, // | vertical
        3 | 7 => 3, // \ diagonal
        _ => 4,     // + fallback
    }
}

/// Perform 3x3 convolution at a single pixel using luminance
fn convolve_3x3_luminance(img: &DynamicImage, x: u32, y: u32, kernel: &[[i32; 3]; 3]) -> i32 {
    let mut sum: i32 = 0;
    
    for ky in 0..3 {
        for kx in 0..3 {
            let px = x + kx - 1;
            let py = y + ky - 1;
            
            let [r, g, b, _] = img.get_pixel(px, py).0;
            
            // Luminance
            let lum = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as i32;
            let kernel_value = kernel[ky as usize][kx as usize];
            
            sum += lum * kernel_value;
        }
    }
    
    sum
}
