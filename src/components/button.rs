use bevy::prelude::*;

use crate::layout::utils::Size;
use crate::Theme;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Component)]
pub enum ButtonState {
    Default,
    Disabled,
    Hover,
    Selected,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Component)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost,
}

#[derive(PartialEq)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

pub struct CustomButton{
    label: String,
    icon: Option<String>,
    photo: Option<String>,
    style: ButtonStyle,
    state: ButtonState,
    size: Size,
    width_style: ButtonWidth,
    alignment: JustifyContent,
}

impl CustomButton {
    pub fn spawn_under<T: Bundle>(
        &self,
        parent: &mut ChildBuilder,
        tag: T,
        theme: &Res<Theme>
    ) {

        let colors = theme.colors.button.colors_from(self.style, self.state);

        let font = theme.fonts.style.label.clone();

        let (button_width, flex_grow) = match self.width_style {
            ButtonWidth::Expand => (Val::Percent(100.0), 1.0),
            ButtonWidth::Hug => (Val::Auto, 0.0),
        };

        let (height, padding, icon_size, icon_pad, font_size) = match self.size {
            Size::Large => (48.0, 24.0, 24.0, 12.0, theme.fonts.size.lg),
            Size::Medium => (32.0, 12.0, 16.0, 4.0, theme.fonts.size.md),
        };

        parent.spawn((
            Button,
            Node {
                flex_grow,
                height: Val::Px(height),
                flex_basis: button_width,
                width: button_width,
                border: UiRect::all(Val::Px(1.0)),
                justify_content: self.alignment,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                padding: UiRect {
                    left: Val::Px(padding),
                    right: Val::Px(padding),
                    ..default()
                },
                ..default()
            },
            BorderColor(colors.outline),
            BorderRadius::MAX,
            BackgroundColor(colors.background),
            self.style,
            self.state,
            tag,
        )).with_children(|button| {

            // === Spawn Icon === //

            if let Some(icon) = &self.icon {
                button.spawn((
                    theme.icons.get(icon),
                    Node {
                        height: Val::Px(icon_size),
                        width: Val::Px(icon_size),
                        margin: UiRect::right(Val::Px(icon_pad)), 
                        ..default()
                    },
                ));
            }

            // if let Some(photo) = self.photo.clone() {
            //     button.spawn(Node::default()).with_children(|parent| {
            //         profile_photo(parent, theme, asset_server, 32.0, &photo);
            //     });
            // }

            // === Spawn Label === //

            button.spawn((
                Text::new(self.label.clone()),
                TextFont {
                    font,
                    font_size,
                    ..default()
                },
                TextColor(colors.label),
            ));     
        });
    }
}

pub fn button_system(
    theme: Res<Theme>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&ButtonStyle>,
            &ButtonState,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, button_style, state) in &mut interaction_query {
        if *state != ButtonState::Disabled && *state != ButtonState::Selected {
            if let Some(button_style) = button_style {
                match *interaction {
                    Interaction::Hovered => {
                        let colors = theme.colors.button.colors_from(*button_style, ButtonState::Hover);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::None => {
                        let colors = theme.colors.button.colors_from(*button_style, ButtonState::Default);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::Pressed => {
                        let colors = theme.colors.button.colors_from(*button_style, ButtonState::Selected);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                }
            }
        }
    }
}

pub fn secondary_default(label: &str, icon: &str) -> CustomButton {
    CustomButton {
        label: label.to_string(),
        icon: Some(icon.to_string()),
        photo: None,
        style: ButtonStyle::Secondary,
        state: ButtonState::Default,
        size: Size::Medium,
        width_style: ButtonWidth::Hug,
        alignment: JustifyContent::Center,
    }
}

pub fn context_button(label: &str, icon: &str) -> CustomButton {
    CustomButton {
        label: label.to_string(),
        icon: Some(icon.to_string()),
        photo: None,
        style: ButtonStyle::Ghost,
        state: ButtonState::Default,
        size: Size::Medium,
        width_style: ButtonWidth::Expand,
        alignment: JustifyContent::Start,
    }
}

pub fn nav_button(label: String, icon: String, state: ButtonState) -> CustomButton {
    CustomButton {
        label,
        icon: Some(icon),
        photo: None,
        style: ButtonStyle::Ghost,
        state,
        size: Size::Large,
        width_style: ButtonWidth::Expand,
        alignment: JustifyContent::Start,
    } 
}

pub fn nav_profile(name: &str, state: ButtonState) -> CustomButton {
    CustomButton {
        label: name.to_string(),
        icon: None,
        photo: Some("profile_picture".to_string()),
        style: ButtonStyle::Ghost,
        state,
        size: Size::Large,
        width_style: ButtonWidth::Expand,
        alignment: JustifyContent::Start,
    } 
}