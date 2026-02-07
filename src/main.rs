//! # img2ascii
//!
//! Entry point for the img2ascii application.
//! This program converts images into ASCII art with support for
//! multiple output formats (terminal, text, HTML, ANSI).

// Module declarations
mod cli;
mod core;
mod output;
mod types;
mod convert;
mod renderansi;
mod renderhtml;
mod edge;

// Main entry point
fn main() -> std::io::Result<()> {
    core::run()
}





