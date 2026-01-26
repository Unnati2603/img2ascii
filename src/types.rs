// ASCII characters ordered from darkest to lightest
pub const ASCII_CHARS: &[u8] = b"@%#*+=-:. ";

// Structure to hold ASCII character and its RGB color
#[derive(Clone, Copy, Debug)]
pub struct AsciiCell {
    pub ch: char,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
