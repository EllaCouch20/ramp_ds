use bevy::prelude::*;

use crate::layout::utils::EXPAND;

use crate::theme::NavigateTo;
use crate::Theme;

use crate::components::button::ButtonState;
use crate::components::button::nav_button;
use crate::components::button::nav_profile;

pub struct Tab {
    pub name: String,
    pub icon: String,
    pub navigate_to: NavigateTo,
    pub selected: bool,
}

#[derive(Component)]
pub struct Navigator {
    logo: Option<String>,
    profile: Option<String>,
    tabs: Vec<Tab>,
}

impl Navigator {
    pub fn new(
        logo: Option<String>,
        profile: Option<String>,
        tabs: Vec<Tab>,
    ) -> Self {
        Self {
            logo,
            profile,
            tabs: tabs.into_iter().map(|mut tab| {
                tab.selected = false;
                tab
            }).collect(),
        }
    }

    pub fn tab_navigator(&self, _parent: &mut ChildBuilder, _theme: &Res<Theme>) {}

    pub fn sidebar_navigator(&self, parent: &mut ChildBuilder, theme: &Res<Theme>) {
        parent.spawn(Node {
            width: Val::Px(300.0),
            height: EXPAND,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(Node {
                width: EXPAND,
                height: EXPAND,
                border: UiRect::right(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(32.0),
                padding: UiRect {
                    top: Val::Px(32.0),
                    bottom: Val::Px(32.0),
                    left: Val::Px(16.0),
                    right: Val::Px(16.0),
                },
                ..default()
            })
            .with_children(|parent| {

                if let Some(logo) = &self.logo {
                    parent.spawn((
                        theme.icons.get(logo),
                        Node {
                            width: Val::Px(90.0),
                            ..default()
                        },
                    ));
                }

                parent.spawn(Node {
                    width: EXPAND,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(8.0),
                    ..default()
                }).with_children(|parent| {
                    for tab in self.tabs.iter(){
                        let is_selected = tab.selected;

                        if is_selected {
                            nav_button(tab.name.clone(), tab.icon.clone(), ButtonState::Selected).spawn_under(parent, tab.navigate_to, theme);
                        } else {
                            nav_button(tab.name.clone(), tab.icon.clone(), ButtonState::Default).spawn_under(parent, tab.navigate_to, theme);
                        }
                    }
                });

                parent.spawn(Node {
                    width: EXPAND,
                    height: EXPAND,
                    ..default()
                });

                if let Some(profile) = &self.profile {
                    nav_profile(profile, ButtonState::Default).spawn_under(parent, NavigateTo::Profile, theme);
                }
            });
        });
    }
}


#[derive(Component)]
pub enum OnClick {
    TabSelected(usize),
}

fn handle_tab_selection(
    mut query: Query<&mut Navigator>, 
    mut interaction_query: Query<(&Interaction, &OnClick)>, 
) {
    for (interaction, click_event) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let OnClick::TabSelected(index) = click_event {
                if let Ok(mut navigator) = query.get_single_mut() {
                    for tab in navigator.tabs.iter_mut() {
                        tab.selected = false;
                    }
                    if let Some(selected_tab) = navigator.tabs.get_mut(*index) {
                        selected_tab.selected = true;
                    }
                }
            }
        }
    }
}
