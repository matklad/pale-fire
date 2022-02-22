mod imp;
mod palette;

use mottle::dsl::ThemeBuilder;
use palette::Palette;

fn main() -> anyhow::Result<()> {
    gen_and_save_theme("Pale Fire", Palette::ORIGINAL)?;
    gen_and_save_theme("Pale Fire High Contrast", Palette::HIGH_CONTRAST)?;
    gen_and_save_theme("Pale Fire Darker", Palette::DARKER)?;
    gen_and_save_theme("Pale Fire Stealth", Palette::STEALTH)?;

    Ok(())
}

fn gen_and_save_theme(name: &str, palette: Palette) -> anyhow::Result<()> {
    let mut theme_builder = ThemeBuilder::default();
    imp::add_rules(&mut theme_builder, palette);

    let theme = theme_builder.build(name);
    mottle::save_theme(&theme)?;

    Ok(())
}
