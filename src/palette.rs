use tincture::{Hue, Oklch};

// Colors are from https://github.com/bbatsov/zenburn_emacs/blob/master/zenburn_theme.el

const FG_CHROMA: f32 = 0.022;
const FG_HUE: f32 = 107.0;

pub(crate) fn fg() -> Oklch {
    oklch(0.89, FG_CHROMA, FG_HUE)
}

pub(crate) fn bright_fg() -> Oklch {
    oklch(0.99, FG_CHROMA, FG_HUE)
}

pub(crate) struct GreyscaleLightness(u32);

impl From<u32> for GreyscaleLightness {
    fn from(lightness: u32) -> Self {
        assert!((0..=6).contains(&lightness));
        Self(lightness)
    }
}

impl From<GreyscaleLightness> for f32 {
    fn from(lightness: GreyscaleLightness) -> Self {
        match lightness.0 {
            0 => 0.3,
            1 => 0.34,
            2 => 0.37,
            3 => 0.4,
            4 => 0.43,
            5 => 0.5,
            6 => 0.55,
            _ => unreachable!(),
        }
    }
}

pub(crate) fn bg(lightness: impl Into<GreyscaleLightness>) -> Oklch {
    oklch(f32::from(lightness.into()), 0.0, 0.0)
}

pub(crate) struct ColorLightness(u32);

impl From<ColorLightness> for f32 {
    fn from(lightness: ColorLightness) -> Self {
        match lightness.0 {
            0 => 0.65,
            1 => 0.75,
            2 => 0.8,
            3 => 0.85,
            4 => 0.9,
            _ => unreachable!(),
        }
    }
}

impl From<u32> for ColorLightness {
    fn from(lightness: u32) -> Self {
        assert!((0..=4).contains(&lightness));
        Self(lightness)
    }
}

pub(crate) enum ColorLightnessPreset {
    TerminalAnsi,
    TerminalAnsiBright,
    DiffFg,
    DiffBg,
    Gutter,
    OverviewRuler,
    GitDecoration,
    Minimap,
}

impl From<ColorLightnessPreset> for ColorLightness {
    fn from(preset: ColorLightnessPreset) -> Self {
        Self(match preset {
            ColorLightnessPreset::TerminalAnsi => 1,
            ColorLightnessPreset::TerminalAnsiBright => 3,
            ColorLightnessPreset::DiffFg => 3,
            ColorLightnessPreset::DiffBg => 0,
            ColorLightnessPreset::Gutter => 1,
            ColorLightnessPreset::OverviewRuler => 2,
            ColorLightnessPreset::GitDecoration => 3,
            ColorLightnessPreset::Minimap => 2,
        })
    }
}

impl From<ColorLightnessPreset> for f32 {
    fn from(preset: ColorLightnessPreset) -> Self {
        let lightness = ColorLightness::from(preset);
        lightness.into()
    }
}

const COLOR_CHROMA: f32 = 0.065;

macro_rules! def_color_fn {
    ($name:ident, hue: $hue:literal) => {
        pub(crate) fn $name(lightness: impl Into<ColorLightness>) -> Oklch {
            oklch(f32::from(lightness.into()), COLOR_CHROMA, $hue)
        }
    };
}

def_color_fn!(red, hue: 19.0);
def_color_fn!(orange, hue: 55.0);
def_color_fn!(yellow, hue: 91.0);
def_color_fn!(green, hue: 145.0);
def_color_fn!(cyan, hue: 200.0);

pub(crate) fn blue(lightness: impl Into<ColorLightness>) -> Oklch {
    let lightness = lightness.into();

    let chroma = if lightness.0 == 4 {
        COLOR_CHROMA * 0.7
    } else {
        COLOR_CHROMA
    };

    oklch(f32::from(lightness), chroma, 243.0)
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h),
    }
}
