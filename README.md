# img2ascii

A Rust project that converts images to ASCII art in the terminal with optional support for custom dimensions and vibrant 24-bit true color output.

## Examples

| Example            | Original                         | ASCII Output                                |
| ------------------ | -------------------------------- | ------------------------------------------- |
| **Circle**         | ![Circle](circle.jpg)            | ![Circle ASCII](circleASCIIoutput.png)      |
| **Cat**            | ![Cat](cat.jpg)                  | ![Cat ASCII](catascii.png)                  |
| **Mona Lisa**      | ![Mona Lisa](monalisa.jpg)       | ![Mona Lisa ASCII](monalisaascii.png)       |
| **Lain**           | ![Lain](lain.jpg)                | ![Lain ASCII](lainascii.png)                |
| **Lain (Colored)** | ![Lain Colored](laincolored.jpg) | ![Lain ASCII Colored](laincoloredascii.png) |

---

## Installation

Install img2ascii using Cargo (Rust package manager):

```bash
cargo install img2ascii-cli
```

This installs the img2ascii command globally.

## Usage

### Basic usage

Convert an image to ASCII art in the terminal:

```bash
img2ascii image.jpg
```

Outputs ASCII art with a default width of 80 characters.

### Set output width

Control the width (in characters) of the ASCII output:

```bash
img2ascii image.jpg --width 60
```

or shorthand:

```bash
img2ascii image.jpg -w 60
```

---

### Tip: Detail & Width

**For more detailed ASCII art output, increase the width!** The larger the width value, the more characters are used to represent the image, which preserves more details and produces a more refined result.

- **Small width (40-60)**: Simpler, bolder appearance
- **Medium width (80-100)**: Balanced detail and visibility
- **Large width (120+)**: Maximum detail and fine features

Example:

```bash
cargo run -- circle.jpg -w 40    # Simple version
cargo run -- circle.jpg -w 120   # Detailed version
```

**Default** (width=80, height auto-calculated with aspect ratio correction):

```bash
cargo run -- circle.jpg
```

**Custom width**:

```bash
cargo run -- circle.jpg --width 100
```

**Custom width and height** (no aspect ratio correction applied):

```bash
cargo run -- circle.jpg --width 100 --height 50
```

**Short flags**:

```bash
cargo run -- circle.jpg -w 100 -H 50
```

**Colored ASCII art**:

```bash
cargo run -- circle.jpg --color
```

**Colored with custom dimensions**:

```bash
cargo run -- circle.jpg -c -w 120 -H 40
```

```

---

## Command-line Flags

| Flag                    | Description                                                                   |
| ----------------------- | ----------------------------------------------------------------------------- |
| `-w, --width <WIDTH>`   | Width of ASCII output in characters _(default: 80)_                           |
| `-H, --height <HEIGHT>` | Height of ASCII output in characters _(optional, overrides auto-calculation)_ |
| `-c, --color`           | Enable colored ASCII art output using ANSI 24-bit true color                  |

---
more img2ascii image.jpg              # → terminal (default)
img2ascii image.jpg -o txt       # → image.txt
img2ascii image.jpg -o html      # → image.html
img2ascii image.jpg -o ansi      # → image.ansi
img2ascii image.jpg --output txt

```

### Prerequisites

Install Rust by running:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation:

```bash
rustc --version
cargo --version
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run -- <image> [OPTIONS]
```

---

## License

This project is for learning and development purposes.

---
