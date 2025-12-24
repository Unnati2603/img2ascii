## Project Structure

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

## Publishing to crates.io

Make sure this exists:

- src/main.rs
- Cargo.toml
- README.md

### Step 1 — Login to crates.io (once only)

You already did this, but for completeness:

```bash
cargo login
```

Paste your crates.io API token (with publish scope).

If already logged in, Cargo will say nothing — that’s fine.

### Step 2 — Dry run (MANDATORY)

This checks:

- metadata
- license
- README
- excludes
- build success

```bash
cargo publish --dry-run
```

✔ If this succeeds → move to Step 3

❌ If it fails → Fix exactly what Cargo tells you (it's very clear).

### Step 3 — Publish

```bash
cargo publish
```

Cargo will:

- Package your crate
- Upload it
- Add it to the crates.io index

### Step 4 — Verify (important)

```bash
cargo search img2ascii-cli
```

see your crate.
Or open its crates.io page.

### Step 5 — Test like a real user

From any directory (or another machine):

```bash
cargo install img2ascii-cli
img2ascii --help
img2ascii image.jpg -w 80 --color
```

If this works → officially published.

### Future updates (remember this)

change code later: Bump version in Cargo.toml

```toml
version = "0.1.1"
```

Run:

```bash
cargo publish
```

You cannot reuse versions.
