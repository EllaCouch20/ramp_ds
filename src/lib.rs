use bevy::prelude::*;

pub mod components;
pub mod theme;

use theme::Theme;

pub struct RampDSPlugin {
    theme: Theme
}

impl RampDSPlugin {
    pub fn new(theme: Theme) -> Self {
        RampDSPlugin{theme}
    }
}

impl Plugin for RampDSPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, startup_system);
    }
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Theme.(&asset_server));
}
