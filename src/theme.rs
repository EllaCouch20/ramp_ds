use bevy::prelude::*;

pub mod color;
pub mod icons;
pub mod fonts;

#[derive(Resource)]
pub struct Theme {
    pub fonts: fonts::FontResources,
    pub colors: color::ColorResources,
    pub icons: icons::IconResources,
    pub app_name: String,
}

impl Theme {
    pub fn new(
        asset_server: Res<AssetServer>,
        theme_template: Res<ThemeTemplate>) -> Self {
        Theme {
            colors: theme_template.colors.clone(),
            fonts: fonts::FontResources::new(&asset_server, theme_template.fonts.clone(), theme_template.font_sizes),
            icons: icons::IconResources::new(asset_server, theme_template.icons.vec.clone()),
            app_name: theme_template.app_name.clone(),
        }
    }
}

#[derive(Resource, Clone)]
pub struct ThemeTemplate {
    pub colors: color::ColorResources,
    pub fonts: fonts::Fonts,
    pub font_sizes: fonts::FontSizes,
    pub icons: icons::Icons,
    pub app_name: String,
}

impl ThemeTemplate {
    pub fn new(
        colors: Option<color::ColorResources>,
        fonts: Option<fonts::Fonts>,
        font_sizes: Option<fonts::FontSizes>,
        icons: Option<icons::Icons>,
        a: &str,
    ) -> Self {
        Self {
            colors: colors.unwrap_or_else(color::ColorResources::default),
            fonts: fonts.unwrap_or_else(fonts::Fonts::default),
            font_sizes: font_sizes.unwrap_or_else(fonts::FontSizes::default),
            icons: icons.unwrap_or_else(icons::Icons::default),
            app_name: a.to_string(),
        }
    }
}

#[derive(Component, Copy, Clone)]
pub enum NavigateTo {
    Bitcoin,
    Messages,
    Profile
}