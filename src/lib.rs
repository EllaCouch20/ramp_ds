use bevy::prelude::*;
use bevy::asset::{LoadedFolder, AssetPath};

pub mod components;
pub mod theme;

use theme::{ThemeTemplate, Theme};

pub struct RampDSPlugin {
    theme_template: ThemeTemplate
}

impl RampDSPlugin {
    pub fn new(theme_template: ThemeTemplate) -> Self {
        RampDSPlugin{theme_template}
    }
}

impl Plugin for RampDSPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.theme_template.clone());
        app.add_systems(PreStartup, pre_startup);
    }
}

fn pre_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<LoadedFolder>>,
    theme_template: Res<ThemeTemplate>
) {
    commands.insert_resource(Theme::new(&asset_server, assets, theme_template));
}
