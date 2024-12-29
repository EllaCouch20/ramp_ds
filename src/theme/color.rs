use bevy::prelude::*;
use std::str::FromStr;

pub struct ColorResources { 
    pub background: BackgroundColor, 
    pub outline: OutlineColor, 
    pub status: StatusColor,
    pub text: TextColor,

    pub icon: IconColor,
    pub button: ButtonColors,
}

impl Default for ColorResources {
    fn default() -> Self {
        ColorResources {
            background: BackgroundColor::default(),
            outline: OutlineColor::default(),
            status: StatusColor::default(),
            text: TextColor::default(),

            icon: IconColor::default(),
            button: ButtonColors::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct BackgroundColor {
    pub primary: Color,
    pub secondary: Color,
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor {
            primary: Colors::grayscale().shade1000,
            secondary: Colors::grayscale().shade950,
        }
    }
}

#[derive(Copy, Clone)]
pub struct OutlineColor {
    pub primary: Color,
    pub secondary: Color,
    pub tint: Color,
}

impl Default for OutlineColor {
    fn default() -> Self {
        OutlineColor {
            primary: Colors::grayscale().shade0,
            secondary: Colors::grayscale().shade700,
            tint: Colors::transparent().shade700,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TextColor {
    pub heading: Color,
    pub primary: Color,
    pub secondary: Color,
}

impl Default for TextColor {
    fn default() -> Self {
        TextColor{
            heading: Colors::tapa().shade0,
            primary: Colors::tapa().shade100,
            secondary: Colors::tapa().shade300,
        }
    }
}

#[derive(Copy, Clone)]
pub struct StatusColor {
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
}

impl Default for StatusColor {
    fn default() -> Self {
        StatusColor{
            success: Colors::malachite().shade500,
            warning: Colors::lightning().shade500,
            danger: Colors::torch_red().shade500,
        }
    }
}


/* -------- ICONS -------- */

#[derive(Copy, Clone)]
pub struct IconColor {
    pub default: Color,
    pub disabled: Color,
    pub hover: Color,
    pub selected: Color,
}

impl Default for IconColor {
    fn default() -> Self {
        IconColor{
            default: Colors::tapa().shade0,
            disabled: Colors::tapa().shade700,
            hover: Colors::tapa().shade200,
            selected: Colors::tapa().shade0,
        }
    }
}

/* -------- INTERACTIVE -------- */



#[derive(Copy, Clone)]
pub struct ButtonColor {
    pub background: Color,
    pub label: Color,
    pub outline: Color,
}

#[derive(Copy, Clone)]
pub struct ButtonColors {
    pub primary_default: ButtonColor,
    pub primary_disabled: ButtonColor,
    pub primary_hover: ButtonColor,
    pub primary_selected: ButtonColor,

    pub secondary_default: ButtonColor,
    pub secondary_disabled: ButtonColor,
    pub secondary_hover: ButtonColor,
    pub secondary_selected: ButtonColor,

    pub ghost_default: ButtonColor,
    pub ghost_disabled: ButtonColor,
    pub ghost_hover: ButtonColor,
    pub ghost_selected: ButtonColor,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors{

            /* -------- PRIMARY -------- */

            primary_default: ButtonColor {
                background: Colors::torch_red().shade500,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            primary_disabled: ButtonColor {
                background: Colors::tapa().shade500,
                label: Colors::tapa().shade1000,
                outline: Colors::transparent().shade0,
            },
            primary_hover: ButtonColor {
                background: Colors::torch_red().shade600,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            primary_selected: ButtonColor {
                background: Colors::torch_red().shade700,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },

            /* -------- SECONDARY -------- */

            secondary_default: ButtonColor {
                background: Colors::transparent().shade0,
                label: Colors::tapa().shade0,
                outline: Colors::tapa().shade700,
            },
            secondary_disabled: ButtonColor {
                background: Colors::tapa().shade500,
                label: Colors::tapa().shade1000,
                outline: Colors::tapa().shade700,
            },
            secondary_hover: ButtonColor {
                background: Colors::tapa().shade950,
                label: Colors::tapa().shade0,
                outline: Colors::tapa().shade700,
            },
            secondary_selected: ButtonColor {
                background: Colors::transparent().shade0,
                label: Colors::tapa().shade0,
                outline: Colors::tapa().shade700,
            },

            /* -------- GHOST -------- */

            ghost_default: ButtonColor {
                background: Colors::transparent().shade0,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            ghost_disabled: ButtonColor {
                background: Colors::transparent().shade0,
                label: Colors::tapa().shade500,
                outline: Colors::transparent().shade0,
            },
            ghost_hover: ButtonColor {
                background: Colors::tapa().shade950,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
            ghost_selected: ButtonColor {
                background: Colors::tapa().shade950,
                label: Colors::tapa().shade0,
                outline: Colors::transparent().shade0,
            },
        }
    }
}

/* -------- SHADES -------- */

pub struct Colors {
    pub shade0: Color,
    pub shade50: Color,
    pub shade100: Color,
    pub shade200: Color,
    pub shade300: Color,
    pub shade400: Color,
    pub shade500: Color,
    pub shade600: Color,
    pub shade700: Color,
    pub shade800: Color,
    pub shade900: Color,
    pub shade950: Color,
    pub shade1000: Color,
}
//TODO: use tailwind css color generator to generate the color palettes 
impl Colors {
    pub fn primary() -> Self {Self::torch_red()}
    pub fn warning() -> Self {Self::lightning()}
    pub fn error() -> Self {Self::torch_red()}
    pub fn success() -> Self {Self::malachite()}
    pub fn grayscale() -> Self {Self::tapa()}

    pub fn transparent() -> Self {
        Colors {
            shade0: hex_transparent("ffffff", 0.),
            shade50: hex_transparent("ffffff", 0.),
            shade100: hex_transparent("ffffff", 25.),
            shade200: hex_transparent("ffffff", 50.),
            shade300: hex_transparent("ffffff", 75.),
            shade400: hex_transparent("ffffff", 100.),
            shade500: hex_transparent("ffffff", 125.),
            shade600: hex_transparent("ffffff", 150.),
            shade700: hex_transparent("ffffff", 175.),
            shade800: hex_transparent("ffffff", 200.),
            shade900: hex_transparent("ffffff", 225.),
            shade950: hex_transparent("ffffff", 225.),
            shade1000: hex_transparent("ffffff", 225.),
        }
    }

    fn tapa() -> Self {
        Colors {
            shade0: hex("ffffff"),
            shade50: hex("f4f3f2"),
            shade100: hex("e2e1df"),
            shade200: hex("c7c4c1"),
            shade300: hex("a7a29d"),
            shade400: hex("8e8781"),
            shade500: hex("78716c"),
            shade600: hex("6d6561"),
            shade700: hex("585250"),
            shade800: hex("4d4846"),
            shade900: hex("443f3f"),
            shade950: hex("262322"),
            shade1000: hex("000000"),
        }
    }

    fn torch_red() -> Self {
        Colors {
            shade0: hex("ffffff"),
            shade50: hex("fef2f2"),
            shade100: hex("fee2e3"),
            shade200: hex("fdcbcd"),
            shade300: hex("fba6a9"),
            shade400: hex("f67377"),
            shade500: hex("eb343a"),
            shade600: hex("da282e"),
            shade700: hex("b71e23"),
            shade800: hex("971d21"),
            shade900: hex("7e1e21"),
            shade950: hex("440b0d"),
            shade1000: hex("000000"),
        }
    }

    fn malachite() -> Self {
        Colors {
            shade0: hex("ffffff"),
            shade50: hex("f1fcf2"),
            shade100: hex("dff9e4"),
            shade200: hex("c0f2ca"),
            shade300: hex("8fe6a1"),
            shade400: hex("57d171"),
            shade500: hex("3ccb5a"),
            shade600: hex("239631"),
            shade700: hex("1f7631"),
            shade800: hex("1d5e2c"),
            shade900: hex("1a4d26"),
            shade950: hex("092a12"),
            shade1000: hex("000000"),
        }
    }

    fn lightning() -> Self {
        Colors {
            shade0: hex("ffffff"),
            shade50: hex("fffdeb"),
            shade100: hex("fefac7"),
            shade200: hex("fdf48a"),
            shade300: hex("fce94d"),
            shade400: hex("fbd924"),
            shade500: hex("f5bd14"),
            shade600: hex("d99106"),
            shade700: hex("b46809"),
            shade800: hex("92500e"),
            shade900: hex("78420f"),
            shade950: hex("452203"),
            shade1000: hex("000000"),
        }
    }
}

pub fn hex(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

pub fn hex_transparent(hex: &str, a: f32) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = f32::from_str(&hex[0..2]).unwrap_or(0.);
    let g = f32::from_str(&hex[2..4]).unwrap_or(0.);
    let b = f32::from_str(&hex[4..6]).unwrap_or(0.);
    Color::srgba(r, g, b, a)
}