use bevy::prelude::*;
use crate::Theme;
use crate::traits::{Parent, Component};

pub const EXPAND: Val = Val::Percent(100.0);
pub const MAX: Val = Val::Px(512.0);

pub enum Size {
    Large,
    Medium,
}

pub struct NewText(pub String, pub Handle<Font>, pub f32, pub Color);

impl Component for NewText {
    fn spawn(self, parent: &mut impl Parent, theme: &Res<Theme>) {
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