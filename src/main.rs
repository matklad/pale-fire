mod palette;
mod style;
mod theme;

use json::ser::PrettyFormatter;
use json::Serializer;
use serde::ser::Serialize;
use std::{fs, io};
use theme::Theme;

fn main() -> io::Result<()> {
    let theme = Theme;
    let json: json::Value = theme.into();

    let serialized_json = {
        let mut buf = Vec::new();
        let pretty_formatter = PrettyFormatter::with_indent(b"\t");
        let mut serializer = Serializer::with_formatter(&mut buf, pretty_formatter);

        json.serialize(&mut serializer).unwrap();

        buf.push(b'\n');

        buf
    };

    fs::write("themes/Pale Fire-color-theme.json", serialized_json)?;

    Ok(())
}
