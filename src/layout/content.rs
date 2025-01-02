use bevy::prelude::*;

use crate::traits::{Component};
use crate::Theme;

pub struct Content {
    pub children: Vec<Box<dyn Component>>,
    pub alignment: JustifyContent,
}

impl Component for Content {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder, theme: &Res<Theme>) {
        parent.spawn(Node {
            justify_content: self.alignment,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            max_width: Val::Px(512.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(24.0),
            padding: UiRect::all(Val::Px(24.0)),
            ..default()
        }).with_children(|parent|{
            self.children.into_iter().map(|component| {
                component.spawn(parent, theme);
            });
        });
    }
}

impl Content {
    pub fn new(alignment: JustifyContent, children: &[impl Component]) -> Self {
        Self {
            alignment,
            children: children.into_iter().map(|c| Box::new(c) as Box<dyn Component>).collect(),
        }
    }
}
