use crate::types::AsciiCell;

/// Renders ASCII cells as ANSI-colored terminal output
pub fn render_ansi(cells: &[Vec<AsciiCell>], color: bool) -> String {
    let mut out = String::new();

    for row in cells {
        for cell in row {
            if color {
                out.push_str(&format!(
                    "\x1b[38;2;{};{};{}m{}\x1b[0m",
                    cell.r, cell.g, cell.b, cell.ch
                ));
            } else {
                out.push(cell.ch);
            }
        }
        out.push('\n');
    }

    out
}
