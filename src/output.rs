use std::path::Path;
use crate::cli::OutputFormat;

// Helper function to derive output filename based on input image path and desired format
pub fn output_filename(image_path: &str, format: &OutputFormat) -> String {
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
