use bevy::prelude::*;

use crate::traits::{Parent, Component};
use crate::layout::utils::{Size, NewText};
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

#[derive(Component)]
pub struct Callback(fn());

pub struct Button{
    label: String,
    icon: Option<String>,
    photo: Option<String>,
    style: ButtonStyle,
    state: ButtonState,
    size: Size,
    width_style: ButtonWidth,
    alignment: JustifyContent,
    on_press: Callback
}

impl Component for Button {
    fn spawn(self, parent: &mut impl Parent, theme: &Res<Theme>) {

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
            Node {
                flex_grow,
                width: button_width,
                height: Val::Px(height),
                flex_basis: button_width,
                justify_content: self.alignment,
                border: UiRect::all(Val::Px(1.0)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                padding: UiRect {
                    left: Val::Px(padding),
                    right: Val::Px(padding),
                    ..default()
                },
                ..default()
            },
            bevy::prelude::Button,
            BorderRadius::MAX,
            BorderColor(colors.outline),
            BackgroundColor(colors.background),
            self.style,
            self.state,
            self.on_press
        )).with_children(|parent| {
            // if let Some(icon) = &self.icon {
            //     parent.spawn((
            //         theme.icons.get(icon),
            //         Node {
            //             height: Val::Px(icon_size),
            //             width: Val::Px(icon_size),
            //             margin: UiRect::right(Val::Px(icon_pad)), 
            //             ..default()
            //         },
            //     ));
            // }

            // if let Some(photo) = self.photo.clone() {
            //     button.spawn(Node::default()).with_children(|parent| {
            //         profile_photo(parent, theme, asset_server, 32.0, &photo);
            //     });
            // }

            NewText(self.label, font, font_size, colors.label).spawn(parent, &theme);
        });
    } 
}

impl Button {
    pub fn system(
        theme: Res<Theme>,
        mut interaction_query: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &ButtonStyle,
                &ButtonState,
                &Callback
            ),
            (Changed<Interaction>, With<bevy::prelude::Button>),
        >,
    ) {
        for (interaction, mut background, mut border, style, state, callback) in &mut interaction_query {
            if *state != ButtonState::Disabled {
                (callback.0)();
            }
            if *state != ButtonState::Disabled && *state != ButtonState::Selected {
    
                let button_state = match interaction {
                    Interaction::Pressed => ButtonState::Selected,
                    Interaction::Hovered => ButtonState::Hover,
                    Interaction::None => ButtonState::Default
                };
    
                let colors = theme.colors.button.colors_from(*style, button_state);
                *background = colors.background.into();
                border.0 = colors.outline;
            }
        }
    }

    pub fn secondary(label: &str, icon: &str, on_press: fn()) -> Self {
        Button{
            label: label.to_string(),
            icon: Some(icon.to_string()),
            photo: None,
            style: ButtonStyle::Secondary,
            state: ButtonState::Default,
            size: Size::Medium,
            width_style: ButtonWidth::Hug,
            alignment: JustifyContent::Center,
            on_press: Callback(on_press)
        }
    }

//     pub fn context(label: &str, icon: &str) -> Self {
//         Button {
//             label: label.to_string(),
//             icon: Some(icon.to_string()),
//             photo: None,
//             style: ButtonStyle::Ghost,
//             state: ButtonState::Default,
//             size: Size::Medium,
//             width_style: ButtonWidth::Expand,
//             alignment: JustifyContent::Start,
//             //tag
//         }
//     }

//     pub fn nav(label: String, icon: String, state: ButtonState) -> Self {
//         Button {
//             label,
//             icon: Some(icon),
//             photo: None,
//             style: ButtonStyle::Ghost,
//             state,
//             size: Size::Large,
//             width_style: ButtonWidth::Expand,
//             alignment: JustifyContent::Start,
//             // tag
//         } 
//     }

//     pub fn nav_profile(name: &str, state: ButtonState) -> Self {
//         Button {
//             label: name.to_string(),
//             icon: None,
//             photo: Some("profile_picture".to_string()),
//             style: ButtonStyle::Ghost,
//             state,
//             size: Size::Large,
//             width_style: ButtonWidth::Expand,
//             alignment: JustifyContent::Start,
//            // tag
//         } 
//     }
}

