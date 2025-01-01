use bevy::prelude::*;

pub mod components;
pub mod layout;
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
        app.add_systems(Update, components::button::button_system);
    }
}

fn pre_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme_template: Res<ThemeTemplate>
) {
    commands.insert_resource(Theme::new(asset_server, theme_template));
}
