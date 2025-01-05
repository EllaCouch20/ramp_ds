use bevy::prelude::*;
use crate::traits::{Component};
use crate::layout::utils::{NewIcon, NewText};
use crate::theme::NavigateTo;
use crate::Theme;

pub struct IconButton(pub Option<(ImageNode, NavigateTo)>);

impl Component for IconButton {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder<'_>, _theme: &Res<Theme>) {
        let node = Node {
            height: Val::Px(32.0),
            width: Val::Px(32.0),
            ..default()
        };

        match self.0 {
            Some(bundle) => parent.spawn((node, bundle)),
            None => parent.spawn(node),
        };
    }
}

#[derive(Component, Clone)]
pub struct IconTextButton(pub String, pub bool);

impl IconTextButton {
    pub fn new (n: &str, t: bool) -> Self {
        Self(n.to_string(), t)
    }
}

impl Component for IconTextButton {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder<'_>, theme: &Res<Theme>) {

        let font = theme.fonts.style.text.clone();
        let font_size = theme.fonts.size.md;
        let icon = if self.1 {"folder"} else {"file"};

        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                height: Val::Auto,
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            bevy::prelude::Button,
            *self.clone(),
        )).with_children(|parent| {
            parent.spawn(Node {
                margin: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                align_items: AlignItems::Center,
                ..default()
            }).with_children(|parent| {
                NewIcon(icon.to_string(), 72.0).box_spawn(parent, &theme);
                NewText(self.0, font, font_size, theme.colors.text.heading).box_spawn(parent, &theme);
            });
        });
    }
}