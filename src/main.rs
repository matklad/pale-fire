mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use palette::Palette;
use std::io;

fn main() -> io::Result<()> {
    gen_and_save_theme("Pale Fire", Palette::ORIGINAL)?;
    gen_and_save_theme("Pale Fire High Contrast", Palette::HIGH_CONTRAST)?;
    gen_and_save_theme("Pale Fire Darker", Palette::DARKER)?;
    gen_and_save_theme("Pale Fire Stealth", Palette::STEALTH)?;

    Ok(())
}

fn gen_and_save_theme(name: &str, palette: Palette) -> io::Result<()> {
    let mut theme_builder = ThemeBuilder::new(name.to_string(), Type::Dark);
    imp::add_rules(&mut theme_builder, palette);

    let theme = theme_builder.build();
    theme.save()
}
