use bevy::prelude::*;

pub const EXPAND: Val = Val::Percent(100.0);
pub const MAX: Val = Val::Px(512.0);

pub enum Size {
    Large,
    Medium,
}

pub fn text(
    parent: &mut ChildBuilder,
    text: &str,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font,
            font_size,
            ..Default::default()
        },
        TextColor(color),
    ));
}
