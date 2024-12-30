use bevy::prelude::*;
use bevy::asset::LoadedFolder;

pub mod color;
mod icons;
mod fonts;

#[derive(Resource)]
pub struct Theme {
    pub fonts: fonts::FontResources,
    pub colors: color::ColorResources,
    pub icons: icons::IconResources,
}

impl Theme {
    pub fn new(
        asset_server: &Res<AssetServer>,
        assets: Res<Assets<LoadedFolder>>,
        theme_template: Res<ThemeTemplate>) -> Self {
        Theme {
            colors: theme_template.colors.clone(),
            fonts: fonts::FontResources::new(asset_server, theme_template.fonts.clone(), theme_template.font_sizes),
            icons: icons::IconResources::new(asset_server, assets)
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct ThemeTemplate{
    colors: color::ColorResources,
    fonts: fonts::Fonts,
    font_sizes: fonts::FontSizes,
}