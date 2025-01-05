use bevy::prelude::*;

pub mod components;
pub mod layout;
pub mod theme;
pub mod traits;

pub mod prelude;

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
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: self.theme_template.app_name.clone(),
                ..default()
            }),
            ..default()
        }));
        app.insert_resource(self.theme_template.clone());
        app.add_systems(PreStartup, pre_startup);
        app.add_plugins(bevy_simple_text_input::TextInputPlugin);
        app.add_systems(Update, components::Button::system);
        app.add_systems(Update, components::context::context_menu);
        app.add_systems(Update, components::text_input::text_input_system.after(bevy_simple_text_input::TextInputSystem));
    }
}

fn pre_startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme_template: Res<ThemeTemplate>
) {
    commands.insert_resource(Theme::new(asset_server, theme_template));
}