use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseButton;

use crate::layout::utils::Separator;
use crate::Theme;
use super::Button;
use crate::traits::Component;

#[derive(Component)]
pub struct ContextMenu;
#[derive(Resource)]
pub struct ContextButtons(pub Option<Vec<Button>>);

pub fn context_menu(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut buttons: ResMut<ContextButtons>,
    mut context_menu_query: Query<(Entity, &Children), With<ContextMenu>>,
    theme: Res<Theme>,
) {
    if let Some(mut buttons) = buttons.0.clone() {
        let window = query_window.single();
        if let Some(cursor_position) = window.cursor_position() {
            if mouse_button.just_pressed(MouseButton::Right) {
                if context_menu_query.is_empty() {
                    commands.spawn((
                        Node {
                            left: Val::Percent((cursor_position.y / window.height()) * 100.0),
                            top: Val::Percent((cursor_position.x / window.width()) * 100.0),
                            width: Val::Px(300.0),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BorderColor(theme.colors.outline.secondary),
                        BackgroundColor(theme.colors.background.primary),
                        BorderRadius::all(Val::Px(8.0)),
                        ContextMenu,
                    )).with_children(|parent| {
                        let last = buttons.pop();

                        for button in buttons {
                            button.box_spawn(parent, &theme);
                        }

                        Separator.box_spawn(parent, &theme);

                        if let Some(last) = last {
                            last.clone().box_spawn(parent, &theme);
                        }
                    });
                }
            }
            
            if mouse_button.just_pressed(MouseButton::Left) {
                for (entity, children) in context_menu_query.iter_mut() {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

