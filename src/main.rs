mod color;
mod theme;

use std::{fs, io};
use theme::Theme;

fn main() -> io::Result<()> {
    let theme = Theme;
    let json = format!("{}", theme);

    fs::write("themes/Pale Fire-color-theme.json", json)?;

    Ok(())
}
