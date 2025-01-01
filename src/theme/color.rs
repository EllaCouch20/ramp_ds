#![allow(unused)]
use bevy::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;
use crate::components::button::{ButtonState, ButtonStyle};

#[derive(Clone)]
pub struct ColorResources { 
    pub background: BackgroundColor, 
    pub outline: OutlineColor, 
    pub status: StatusColor,
    pub text: TextColor,
    pub button: ButtonColors,
}

impl Default for ColorResources {
    fn default() -> Self {
        ColorResources {
            background: BackgroundColor::default(),
            outline: OutlineColor::default(),
            status: StatusColor::default(),
            text: TextColor::default(),
            button: ButtonColors::new(
                ButtonSchemes::default(),
            ),
        } 
    }
}

impl ColorResources {
    fn new(
        background: BackgroundColor,
        outline: OutlineColor,
        status: StatusColor,
        text: TextColor,
        button: ButtonColors,
    ) -> Self {
        ColorResources {
            background,
            outline,
            status,
            text,
            button,
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
            primary: hex("000000"),
            secondary: hex("262322"),
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
            primary: hex("ffffff"),
            secondary: hex("585250"),
            tint: hex("585250"),
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
            heading: hex("ffffff"),
            primary: hex("e2e1df"),
            secondary: hex("a7a29d"),
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
            success: hex("3ccb5a"),
            warning: hex("f5bd14"),
            danger: hex("eb343a"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ButtonColorScheme {
    pub background: Color,
    pub label: Color,
    pub outline: Color,
}

#[derive(Copy, Clone)]
pub struct ButtonSchemes {
    pub primary_default: ButtonColorScheme,
    pub primary_disabled: ButtonColorScheme,
    pub primary_hover: ButtonColorScheme,
    pub primary_selected: ButtonColorScheme,

    pub secondary_default: ButtonColorScheme,
    pub secondary_disabled: ButtonColorScheme,
    pub secondary_hover: ButtonColorScheme,
    pub secondary_selected: ButtonColorScheme,

    pub ghost_default: ButtonColorScheme,
    pub ghost_disabled: ButtonColorScheme,
    pub ghost_hover: ButtonColorScheme,
    pub ghost_selected: ButtonColorScheme,
}

impl Default for ButtonSchemes {
    fn default() -> Self {
        ButtonSchemes {
            primary_default: ButtonColorScheme {
                background: hex("eb343a"),
                label: hex("ffffff"),
                outline: transparent(0.),
            },
            primary_disabled: ButtonColorScheme {
                background: hex("eb343a"),
                label: hex("000000"),
                outline: transparent(0.),
            },
            primary_hover: ButtonColorScheme {
                background: hex("da282e"),
                label: hex("ffffff"),
                outline: transparent(0.),
            },
            primary_selected: ButtonColorScheme {
                background: hex("b71e23"),
                label: hex("ffffff"),
                outline: transparent(0.),
            },

            secondary_default: ButtonColorScheme {
                background: transparent(0.),
                label: hex("ffffff"),
                outline: hex("585250"),
            },
            secondary_disabled: ButtonColorScheme {
                background: hex("78716c"),
                label: hex("000000"),
                outline: hex("585250"),
            },
            secondary_hover: ButtonColorScheme {
                background: hex("262322"),
                label: hex("ffffff"),
                outline: hex("585250"),
            },
            secondary_selected: ButtonColorScheme {
                background: transparent(0.),
                label: hex("ffffff"),
                outline: hex("585250"),
            },

            ghost_default: ButtonColorScheme {
                background: transparent(0.),
                label: hex("ffffff"),
                outline: transparent(0.),
            },
            ghost_disabled: ButtonColorScheme {
                background: transparent(0.),
                label: hex("78716c"),
                outline: transparent(0.),
            },
            ghost_hover: ButtonColorScheme {
                background: hex("262322"),
                label: hex("ffffff"),
                outline: transparent(0.),
            },
            ghost_selected: ButtonColorScheme {
                background: hex("262322"),
                label: hex("ffffff"),
                outline: transparent(0.),
            },
        }
    }
}

#[derive(Default, Clone)]
pub struct ButtonColors {
    color_map: HashMap<(ButtonState, ButtonStyle), ButtonColorScheme>,
}

impl ButtonColors {
    fn new(schemes: ButtonSchemes) -> Self {
        let mut color_map = HashMap::new();

        color_map.insert((ButtonState::Default, ButtonStyle::Primary), schemes.primary_default);

        color_map.insert((ButtonState::Disabled, ButtonStyle::Primary), schemes.primary_disabled);

        color_map.insert((ButtonState::Hover, ButtonStyle::Primary), schemes.primary_hover,
        );

        color_map.insert(
            (ButtonState::Selected, ButtonStyle::Primary),
            schemes.primary_selected,
        );

        color_map.insert((ButtonState::Default, ButtonStyle::Secondary), schemes.secondary_default);

        color_map.insert((ButtonState::Disabled, ButtonStyle::Secondary), schemes.secondary_disabled);

        color_map.insert((ButtonState::Hover, ButtonStyle::Secondary), schemes.secondary_hover);

        color_map.insert(
            (ButtonState::Selected, ButtonStyle::Secondary),
            schemes.secondary_selected,
        );

        color_map.insert(
            (ButtonState::Default, ButtonStyle::Ghost),
            schemes.ghost_default,
        );

        color_map.insert(
            (ButtonState::Disabled, ButtonStyle::Ghost),
            schemes.ghost_disabled,
        );

        color_map.insert(
            (ButtonState::Hover, ButtonStyle::Ghost),
            schemes.ghost_hover,
        );

        color_map.insert(
            (ButtonState::Selected, ButtonStyle::Ghost),
            schemes.ghost_selected,
        );

        ButtonColors{ color_map }
    }

    pub fn colors_from(&self, style: ButtonStyle, state: ButtonState) -> ButtonColorScheme {
        self.color_map.get(&(state, style)).copied().expect("ColorScheme Not Found")
    }
}

pub fn hex(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

pub fn transparent(a: f32) -> Color {
    let hex = "ffffff";
    let hex = hex.trim_start_matches('#');
    let r = f32::from_str(&hex[0..2]).unwrap_or(0.);
    let g = f32::from_str(&hex[2..4]).unwrap_or(0.);
    let b = f32::from_str(&hex[4..6]).unwrap_or(0.);
    Color::srgba(r, g, b, a)
}