use bevy::prelude::*;

use crate::Theme;

use crate::traits::Component;

use super::utils::EXPAND;

use super::header::Header;
use super::content::Content;
use super::bumper::Bumper;

pub struct Interface {
    page: Page,
    navigator: bool,
}

impl Interface {
    pub fn new(navigator: bool, page: Page) -> Self {
        Interface{navigator, page}
    }
}

impl Component for Interface {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder, theme: &Res<Theme>) {
        parent.spawn((
            Node {
                width: EXPAND,
                height: EXPAND,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            Interaction::None,
            BackgroundColor(theme.colors.background.primary)
        )).with_children(|parent|{
            self.page.box_spawn(parent, theme);
            // Sidebar
        });
    }
}

pub struct Page {
    header: Header,
    content: Content,
    bumper: Option<Bumper>,
}

impl Page {
    pub fn new(header: Header, content: Content, bumper: Option<Bumper>) -> Self {
        Page{header, content, bumper}
    }
}

impl Component for Page {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder, theme: &Res<Theme>){
        parent.spawn(Node {
            width: EXPAND,
            height: EXPAND,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        }).with_children(|parent| {
            self.header.box_spawn(parent, theme);
            self.content.box_spawn(parent, theme);
            if let Some(bumper) = self.bumper {bumper.box_spawn(parent, theme);}
        });
    }
}


//if MOBILE && self.navigator { nav_system.tab_navigator(parent, theme); }
// let nav_system = Navigator::new(
//     Some("wordmark".to_string()),
//     Some("Ella Couch".to_string()),
//     vec![Tab{name: "Bitcoin".to_string(), icon: "wallet".to_string(), navigate_to: NavigateTo::Bitcoin, selected: true}],
// );
