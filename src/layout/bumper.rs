use bevy::prelude::*;

use crate::Theme;
use crate::theme::NavigateTo;
use crate::layout::utils::EXPAND;
use crate::components::button::CustomButton;

pub struct Bumper {
    content: Vec<(CustomButton, NavigateTo)>,
}

impl Bumper {
    pub fn spawn_under(&self, parent: &mut ChildBuilder, theme: &Res<Theme>) {
        parent.spawn(Node {
            width: EXPAND,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(8.0),
            padding: UiRect {
                top: Val::Px(16.0),
                bottom: Val::Px(16.0),
                left: Val::Px(24.0),
                right: Val::Px(24.0)
            },
            ..default()
        }).with_children(|parent| {
            for (child, nav) in &self.content {
                child.spawn_under(parent, *nav, theme);
            }
        });
    }
}