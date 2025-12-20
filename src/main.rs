use image::{GenericImageView, imageops::FilterTypes};
// GenericImageView trait gives .dimensions() and .get_pixel() methods
// FilterType resize

use clap::Parser;
// Enables Args::parse()
// auto gen CLI parsing code

const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";
// for smoother
// const ASCII_CHARS: &[u8] =  b"$@B%8&WM#*oahkbdpqwmZ0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`. ";
//take mono space font into account


#[derive(Parser)]

// Use clap to turn this struct into a command-line parser
// Describe your arguments as a struct, and I’ll generate the parsing code

struct Args {
    image: String,
    width: u32,
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




// MAIN FUNCTION BOMBOMBOOOOOOO

fn main() {
    let args=Args::parse();         //terminal arg read, parsed, validated into Args struct
    let img=image::open(&args.image).expect("Failed to open image");            //open image file

    let (l,b)=img.dimensions():
    let aspect_ratio= b as f32 / l as f32;          //preserve aspect ratio

}
