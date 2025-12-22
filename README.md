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

---

## Command-line Flags

| Flag                    | Description                                                                   |
| ----------------------- | ----------------------------------------------------------------------------- |
| `-w, --width <WIDTH>`   | Width of ASCII output in characters _(default: 80)_                           |
| `-H, --height <HEIGHT>` | Height of ASCII output in characters _(optional, overrides auto-calculation)_ |
| `-c, --color`           | Enable colored ASCII art output using ANSI 24-bit true color                  |

---

## Getting Started

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

## Usage Examples

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

---

## Project Structure

```
img2ascii/
├── Cargo.toml          ← dependencies and project metadata
└── src/
    └── main.rs         ← main application code
```

---

## How It Works

### Brightness Calculation

The algorithm converts each pixel to a brightness value using the standard luminosity formula:

```
brightness = 0.299 × R + 0.587 × G + 0.114 × B
```

(Red is weighted less because human eyes are less sensitive to red light)

### Character Mapping

Brightness values are mapped to ASCII characters from darkest to lightest:

```
@ % # * + = - : .   (space)
```

- **Dark pixels** → `@`, `%`, `#` (dense characters)
- **Light pixels** → `.`, ` ` (sparse characters)

### Aspect Ratio Correction

Terminal characters are typically taller than they are wide (≈2:1 ratio), so a correction factor of `0.43` is applied to prevent circles from appearing as ovals.

### Color Output

When `--color` is enabled, each character is colored using **ANSI 24-bit true color** escape codes:

```
\x1b[38;2;R;G;Bm{char}\x1b[0m
```

This matches the original image's RGB values for each pixel.

---

## Dependencies

**Cargo.toml** is the configuration + metadata file for a Rust project.
like package.json (Node)

### Main Sections

1. **`[package]`** — Project metadata (required)

   - `name`: Project identifier
   - `version`: Semantic versioning (major.minor.patch)
   - `edition`: Rust edition

2. **`[dependencies]`** — External libraries

   - Libraries are automatically downloaded from [crates.io](https://crates.io)
   - Versions follow semantic versioning

3. **`[bin]`** -- binary target configuration (optional)
   name = "ascii" -- binary name
   path = "src/main.rs" -- source file path
   ......custom executable names.
4. **`[features]`** -- optional features (optional)
   default = [] -- default features
   colored_output = [] -- custom feature
   ......enable/disable optional functionality in dependencies.

### Libraries Used

- **`image`** — Image loading and processing
- **`clap`** — Command-line argument parsing

---

## Rust Basics

### Variables & Mutability

By default, variables are immutable. Use `mut` to make them mutable:

```rust
let x = 5;           // immutable
let mut y = 10;      // mutable
y += 5;              // y is now 15
```

### Data Types

- Integer: i32, u32, i64, u64
- Floating-point: f32, f64
- Boolean: bool (true, false)
- Character: char ('a', '1', '😊')
- String: String (growable, heap-allocated)

```rust
let a: i32 = 10;                              // integer
let b: f64 = 3.14;                            // float
let c: bool = true;                           // boolean
let d: char = 'R';                            // character
let s: String = String::from("Hello, Rust"); // string
```

### Functions

defined using the fn keyword.

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

let result = add(5, 10);  // result is 15
```

### Control Flow

```rust
// if/else
if number < 5 {
    println!("Less than 5");
} else if number == 5 {
    println!("Equal to 5");
} else {
    println!("Greater than 5");
}

// for loop
for i in 0..5 {
    println!("{}", i);  // prints 0 to 4
}

// while loop
let mut count = 0;
while count < 5 {
    println!("{}", count);
    count += 1;
}
```

### Printing

```rust
println!("Hello, {}!", "world");  // Hello, world!
```

### Ownership & Borrowing

- Each value has a single owner.
- When the owner goes out of scope, the value is dropped.
- References allow borrowing values without taking ownership.

Rust ensures memory safety without garbage collection:

```rust
let s1 = String::from("hello");  // s1 owns the string
let s2 = &s1;                     // s2 borrows s1
println!("{}", s2);               // prints "hello"
// s1 can still be used here
```

### Structs

```rust
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 10, y: 20 };
println!("Point({}, {})", p.x, p.y);  // Point(10, 20)
```

### Enums

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let dir = Direction::Up;
match dir {
    Direction::Up => println!("Going up!"),
    Direction::Down => println!("Going down!"),
    Direction::Left => println!("Going left!"),
    Direction::Right => println!("Going right!"),
}
```

---

## License

This project is for learning and development purposes.
