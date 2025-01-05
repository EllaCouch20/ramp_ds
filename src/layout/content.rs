use bevy::prelude::*;

use crate::traits::{Component};
use crate::Theme;

pub struct Content (
    pub JustifyContent,
    pub Vec<Box<dyn Component>>,
);

impl Component for Content {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder, theme: &Res<Theme>) {
        parent.spawn(Node {
            justify_content: self.0,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            max_width: Val::Px(512.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(24.0),
            padding: UiRect::all(Val::Px(24.0)),
            ..default()
        }).with_children(|parent|{
            self.1.into_iter().for_each(|component| {
                component.spawn(parent, theme);
            });
        });
    }
}
