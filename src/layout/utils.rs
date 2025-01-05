use bevy::prelude::*;
use crate::Theme;
use crate::traits::{Component};

pub const EXPAND: Val = Val::Percent(100.0);
pub const MAX: Val = Val::Px(512.0);

#[derive(Clone)]
pub enum Size {
    Large,
    Medium,
}

pub struct NewText(pub String, pub Handle<Font>, pub f32, pub Color);

impl Component for NewText {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder, _theme: &Res<Theme>) {
        parent.spawn((
            Text::new(&self.0),
            TextFont {
                font: self.1,
                font_size: self.2,
                ..default()
            },
            TextColor(self.3),
        ));
    }
}

pub struct NewIcon(pub String, pub f32);

impl Component for NewIcon {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder, theme: &Res<Theme>){
        parent.spawn((
            theme.icons.get(&self.0),
            Node {
                height: Val::Px(self.1),
                width: Val::Px(self.1),
                ..default()
            },
        ));
    }
}

pub struct Padding(pub f32);

impl Component for Padding {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder, _theme: &Res<Theme>){
        parent.spawn(
            Node {
                height: Val::Px(self.0),
                width: Val::Px(self.0),
                ..default()
            },
        );
    }
}

pub struct Separator;

impl Component for Separator {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder, theme: &Res<Theme>){
        parent.spawn((
            Node {
                width: EXPAND,
                height: Val::Px(1.0),
                ..default()
            },
            BackgroundColor(theme.colors.outline.secondary),
        ));
    }
}