use bevy::prelude::*;
use bevy_ui::FocusPolicy;

use crate::traits::Component;
use crate::Theme;

use bevy_simple_text_input::{
    TextInputInactive,
    TextInputTextColor,
    TextInputTextFont,
    TextInputPlaceholder,
    TextInputSubmitEvent,
    TextInputValue
};

pub struct TextInput(String);

impl TextInput {
    pub fn new(txt: &str) -> Self {
        TextInput(txt.to_string())
    }
}

impl Component for TextInput {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder<'_>, theme: &Res<Theme>) {
        parent.spawn((
            Node {
                border: UiRect::all(Val::Px(1.0)),
                height: Val::Px(48.0), 
                width: Val::Percent(100.0),
                align_items: AlignItems::Center, 
                justify_content: JustifyContent::Start,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            TextInputTextFont(TextFont {
                font: theme.fonts.style.text.clone(),
                font_size: theme.fonts.size.md,
                ..default()
            }),
            BorderColor(theme.colors.outline.secondary),
            BackgroundColor(theme.colors.background.primary),
            TextInputTextColor(TextColor(theme.colors.text.primary)),
            TextInputInactive(true),
            TextInputValue(self.0),
            BorderRadius::all(Val::Px(8.0)),
            FocusPolicy::Block,
            bevy_simple_text_input::TextInput,
        ));
    }
}

#[derive(Component, Clone)]
pub struct TextEditor(String);

impl TextEditor {
    pub fn new(txt: &str) -> Self {
        TextEditor(txt.to_string())
    }
}

impl Component for TextEditor {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder<'_>, theme: &Res<Theme>) {
        parent.spawn((
            Node {
                border: UiRect::all(Val::Px(1.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                max_height: Val::Px(1000.0),
                align_items: AlignItems::Start, 
                justify_content: JustifyContent::Start,
                padding: UiRect::all(Val::Px(16.0)),
                ..default()
            },
            TextInputTextFont(TextFont {
                font: theme.fonts.style.text.clone(),
                font_size: theme.fonts.size.md,
                ..default()
            }),
            TextInputPlaceholder {
                value: "Write to file...".to_string(),
                ..default()
            },
            *self.clone(),
            BorderColor(theme.colors.outline.secondary),
            BackgroundColor(theme.colors.background.primary),
            TextInputTextColor(TextColor(theme.colors.text.primary)),
            TextInputInactive(true),
            TextInputValue(self.0),
            BorderRadius::all(Val::Px(8.0)),
            FocusPolicy::Block,
            bevy_simple_text_input::TextInput,
        ));
    }
}

pub fn text_input_system(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
    mut events: EventReader<TextInputSubmitEvent>,
    mut editor_query: Query<&mut TextInputValue, With<TextEditor>>,
    theme: Res<Theme>,
) {

    // On Pressed
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = theme.colors.outline.primary.into();
                } else {
                    inactive.0 = true;
                    *border_color = theme.colors.outline.secondary.into();
                }
            }
        }
    }

    // On Enter
    for event in events.read() {
        for mut text_input in &mut editor_query {
            text_input.0 = event.value.clone();
            text_input.0.push('\n');
        }
    }
    
}
