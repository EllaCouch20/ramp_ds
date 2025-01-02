use bevy::prelude::*;

use crate::traits::{Component};

use crate::Theme;
use crate::theme::NavigateTo;
use crate::layout::utils::EXPAND;
use crate::components::button::Button;

pub struct Bumper(Button, Option<Button>);

impl Component for Bumper {
    fn spawn(&self, parent: &mut ChildBuilder, theme: &Res<Theme>) {
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
            self.0.spawn(parent, theme);

            if let Some(button) = self.1 {
                button.spawn(parent, theme);
            }
        });
    }
}
