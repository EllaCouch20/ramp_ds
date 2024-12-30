use bevy::prelude::*;
use bevy::asset::{LoadedFolder, AssetPath};

mod icons;
mod color;
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
            colors: color::ColorResources::default(),
            fonts: fonts::FontResources::new(&asset_server),
            icons: icons::IconResources::new(&asset_server, assets)
        }
    }
}

#[derive(Resource, Clone)]
pub struct ThemeTemplate{

}
