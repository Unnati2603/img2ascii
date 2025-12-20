use image::GenericImageView;
// GenericImageView trait gives .dimensions() and .get_pixel() methods

use clap::Parser;
// Enables Args::parse()
// auto gen CLI parsing code

// const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";

// for smoother
// const ASCII_CHARS: &[u8] =  b"$@B%8&WM#*oahkbdpqwmZ0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`. ";
//take mono space font into account
const ASCII_CHARS: &[u8] =  b"$@B%8&WM#*/\\|()1{}[]?-_+~<>i!lI;:,\"^`. ";

#[derive(Parser)]

// Use clap to turn this struct into a command-line parser
// Describe your arguments as a struct, and I’ll generate the parsing code

struct Args {
    image: String,
    #[arg(short, long, default_value_t = 80)]
    width: u32,
    #[arg(short = 'H', long)]
    height: Option<u32>,
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

    let (w,h)=img.dimensions();
    let w = w as f32;
    let h = h as f32;
    let aspect_ratio = h as f32/w as f32;          //preserve aspect ratio, interger division bad hence float 

    // let (width, height) = img.dimensions();
    // let aspect_ratio = height as f32 / width as f32;

    let new_w = args.width;       //desired width input as a flag

    // let new_h=(aspect_ratio* new_w as f32) as u32;

    let char_aspect = 0.43;
    let mut new_h=(new_w as f32 * aspect_ratio*char_aspect ) as u32;
    // most terminal char are not square, they are taller than they are wider. but img pixels are square.
    // typical terminal chars are ~2:1 (height:width), so we need ~0.4-0.5 correction
    // adjust this value if circle still looks like oval
    
    // override with custom height if provided
    if let Some(h) = args.height {
        new_h = h;
    }
    use image::imageops::FilterType;
    // Bring the FilterType enum into scope

    let resized_img = img.resize_exact(
        new_w, new_h, FilterType::Nearest,
    );
    // img.resize_exact(width, height, filter)
    // resize_exact forces the exact dimensions (ignores original aspect ratio)
    // we handle aspect ratio ourselves with char_aspect correction
    /*
    Nearest Neighbor resizing
    For every pixel in the new image:
    1. Find the closest pixel in the old image
    2. Copy its value directly
    3. No blending, no smoothing
    */

    let grey_img= resized_img.grayscale();         //ascii depends on brightness

    // loop over all pixels
    for y in 0..grey_img.height(){
        for x in 0..grey_img.width(){
            let pixel=grey_img.get_pixel(x,y)[0];                       //brightness
            let unnati= (pixel as usize * ASCII_CHARS.len().saturating_sub(1))/256;       // mapping, out of bonds
            print!("{}", ASCII_CHARS[unnati] as char);
        }
        println!();             //newline after each row
    }


}
