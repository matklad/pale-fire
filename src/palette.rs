use tincture::{Hue, Oklch};

pub(crate) struct Palette {
    base_foreground_lightness: f32,
    base_greyscale_lightness: f32,
    base_color_lightness: f32,
    color_chroma: f32,
}

impl Palette {
    pub(crate) const ORIGINAL: Self = Self {
        base_foreground_lightness: 0.89,
        base_greyscale_lightness: 0.37,
        base_color_lightness: 0.8,
        color_chroma: 0.065,
    };

    pub(crate) const HIGH_CONTRAST: Self = Self {
        base_foreground_lightness: 0.93,
        base_greyscale_lightness: 0.34,
        base_color_lightness: 0.81,
        color_chroma: 0.077,
    };

    const FG_CHROMA: f32 = 0.025;
    const FG_HUE: f32 = 107.0;

    pub(crate) fn fg(&self) -> Oklch {
        oklch(
            self.base_foreground_lightness,
            Self::FG_CHROMA,
            Self::FG_HUE,
        )
    }

    pub(crate) fn bright_fg(&self) -> Oklch {
        oklch(
            (self.base_foreground_lightness + 0.1).min(0.99),
            Self::FG_CHROMA,
            Self::FG_HUE,
        )
    }

    pub(crate) fn greyscale(&self, lightness: impl Into<GreyscaleLightness>) -> Oklch {
        oklch(self.greyscale_lightness(lightness), 0.0, 0.0)
    }

    fn greyscale_lightness(&self, lightness: impl Into<GreyscaleLightness>) -> f32 {
        let lightness = lightness.into();

        match lightness.0 {
            -2 => self.base_greyscale_lightness - 0.07,
            -1 => self.base_greyscale_lightness - 0.03,
            0 => self.base_greyscale_lightness,
            1 => self.base_greyscale_lightness + 0.03,
            2 => self.base_greyscale_lightness + 0.06,
            3 => self.base_greyscale_lightness + 0.13,
            4 => self.base_greyscale_lightness + 0.18,
            5 => self.base_greyscale_lightness + 0.33,
            _ => unreachable!(),
        }
    }

    fn color_lightness(&self, lightness: impl Into<ColorLightness>) -> f32 {
        let lightness = lightness.into();

        match lightness.0 {
            0 => self.base_color_lightness - 0.15,
            1 => self.base_color_lightness - 0.05,
            2 => self.base_color_lightness,
            3 => self.base_color_lightness + 0.05,
            4 => self.base_color_lightness + 0.1,
            _ => unreachable!(),
        }
    }
}

macro_rules! def_color_method {
    ($name:ident, hue: $hue:literal) => {
        impl Palette {
            pub(crate) fn $name(&self, lightness: impl Into<ColorLightness>) -> Oklch {
                oklch(self.color_lightness(lightness), self.color_chroma, $hue)
            }
        }
    };
}

def_color_method!(red, hue: 19.0);
def_color_method!(orange, hue: 55.0);
def_color_method!(yellow, hue: 91.0);
def_color_method!(green, hue: 145.0);
def_color_method!(cyan, hue: 200.0);

impl Palette {
    pub(crate) fn blue(&self, lightness: impl Into<ColorLightness>) -> Oklch {
        let lightness = lightness.into();

        let chroma = if lightness.0 == 4 {
            self.color_chroma.min(0.045)
        } else {
            self.color_chroma
        };

        oklch(self.color_lightness(lightness), chroma, 243.0)
    }
}

pub(crate) struct GreyscaleLightness(i32);

impl From<i32> for GreyscaleLightness {
    fn from(lightness: i32) -> Self {
        assert!((-2..=5).contains(&lightness));
        Self(lightness)
    }
}

pub(crate) struct ColorLightness(u32);

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
    StatusBar,
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
            ColorLightnessPreset::StatusBar => 1,
        })
    }
}

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}
