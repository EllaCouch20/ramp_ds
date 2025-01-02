use bevy::prelude::*;

use crate::layout::utils::EXPAND;
use crate::traits::{Component};

use crate::theme::NavigateTo;
use crate::Theme;

use crate::components::button::ButtonState;
use crate::components::button::Button;

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
}

impl Component for Navigator {
    //fn spawn_tab(self, _parent: &mut impl Parent, _theme: &Res<Theme>) {}

    fn spawn(&self, parent: &mut ChildBuilder, theme: &Res<Theme>) {
        // parent.spawn(Node {
        //     width: Val::Px(300.0),
        //     height: EXPAND,
        //     align_items: AlignItems::Start,
        //     justify_content: JustifyContent::Start,
        //     ..default()
        // }).with_children(|parent| {
        //     parent.spawn(Node {
        //         width: EXPAND,
        //         height: EXPAND,
        //         border: UiRect::right(Val::Px(1.0)),
        //         justify_content: JustifyContent::Center,
        //         align_items: AlignItems::Center,
        //         flex_direction: FlexDirection::Column,
        //         row_gap: Val::Px(32.0),
        //         padding: UiRect {
        //             top: Val::Px(32.0),
        //             bottom: Val::Px(32.0),
        //             left: Val::Px(16.0),
        //             right: Val::Px(16.0),
        //         },
        //         ..default()
        //     }).with_children(|parent| {

        //         if let Some(logo) = &self.logo {
        //             parent.spawn((
        //                 theme.icons.get(logo),
        //                 Node {
        //                     width: Val::Px(90.0),
        //                     ..default()
        //                 },
        //             ));
        //         }

        //         parent.spawn(Node {
        //             width: EXPAND,
        //             align_items: AlignItems::Center,
        //             flex_direction: FlexDirection::Column,
        //             row_gap: Val::Px(8.0),
        //             ..default()
        //         }).with_children(|parent| {
        //             for tab in self.tabs.iter(){
        //                 let state = if tab.selected {ButtonState::Selected} else {ButtonState::Default};
        //                 // tab.navigate_to, 
        //                 Button::nav(tab.name.clone(), tab.icon.clone(), state).spawn(parent, theme);
        //             }
        //         });

        //         parent.spawn(Node {
        //             width: EXPAND,
        //             height: EXPAND,
        //             ..default()
        //         });

        //         // if let Some(profile) = &self.profile {
        //         //     let state = if tab.selected {ButtonState::Selected} else {ButtonState::Default};
        //         //     Button::nav(NavigateTo::Profile, tab.name.clone(), tab.icon.clone(), state).spawn(parent, theme)
        //         // }
        //     });
        // });
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
