use bevy::prelude::*;
use crate::components::button::Button;
pub mod color;
pub mod icons;
pub mod fonts;

#[derive(Resource)]
pub struct Theme {
    pub fonts: fonts::FontResources,
    pub colors: color::ColorResources,
    pub icons: icons::IconResources,
    pub context_menu: Option<Vec<Button>>,
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
            context_menu: theme_template.context_menu.clone(),
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
    pub context_menu: Option<Vec<Button>>,
}

impl ThemeTemplate {
    pub fn new(
        colors: Option<color::ColorResources>,
        fonts: Option<fonts::Fonts>,
        font_sizes: Option<fonts::FontSizes>,
        icons: Option<icons::Icons>,
        app_name: String,
        context_menu: Option<Vec<Button>>,
    ) -> Self {
        Self {
            colors: colors.unwrap_or_else(color::ColorResources::default),
            fonts: fonts.unwrap_or_else(fonts::Fonts::default),
            font_sizes: font_sizes.unwrap_or_else(fonts::FontSizes::default),
            icons: icons.unwrap_or_else(icons::Icons::default),
            app_name,
            context_menu,
        }
    }
}

#[derive(Component, Copy, Clone)]
pub enum NavigateTo {
    Bitcoin,
    Messages,
    Profile
}