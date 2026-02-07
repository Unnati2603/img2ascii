//! Command-line Interface Module
//!
//! Defines the command-line argument structure and output format options
//! for the img2ascii application using the clap parser.

use clap::{Parser, ValueEnum};

// Define the output format enum
#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Txt,
    Html,
    Ansi,
}

/// Command-line arguments for img2ascii
#[derive(Debug, Parser)]
#[command(version)]
pub struct Args {
    /// Path to the input image file
    #[arg(value_name = "IMAGE", index = 1)]
    pub image: String,

    /// Output width in characters
    #[arg(short, long, default_value_t = 80)]
    pub width: u32,

    /// Output height in characters (overrides aspect ratio)
    #[arg(short = 'H', long)]
    pub height: Option<u32>,

    /// Enable colored ASCII output
    #[arg(short, long)]
    pub color: bool,

    /// Output format (txt, html, ansi). If omitted, prints to terminal.
    #[arg(short = 'o', long = "output", value_enum)]
    pub output: Option<OutputFormat>,

    /// Apply Sobel edge detection before conversion
    #[arg(short = 'e', long)]
    pub edges: bool,

    /// Edge detection threshold (0-255)
    #[arg(long, default_value_t = 100)]
    pub edge_threshold: u8,
}  

