// Module declarations
mod cli;
mod core;
mod output;
mod types;
mod convert;
mod renderansi;
mod renderhtml;

// Main entry point
fn main() -> std::io::Result<()> {
    core::run()
}





