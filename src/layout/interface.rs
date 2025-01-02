use bevy::prelude::*;

use crate::Theme;
use crate::theme::NavigateTo;

use crate::traits::{Parent, Component};

use super::utils::EXPAND;

use super::header::Header;
use super::content::Content;
use super::bumper::Bumper;
use super::navigator::Tab;
use super::navigator::Navigator;

const DESKTOP: bool = true;
const MOBILE: bool = false;



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
    fn spawn(self, parent: &mut impl Parent, theme: &Res<Theme>) {
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
            self.page.spawn(parent, theme);
            //if DESKTOP && self.navigator { nav_system.sidebar_navigator(parent, theme); }
            //self.page.spawn_under(parent, theme);
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
    fn spawn(self, parent: &mut impl Parent, theme: &Res<Theme>){
        parent.spawn(Node {
            width: EXPAND,
            height: EXPAND,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        }).with_children(|parent| {
            self.header.spawn(parent, theme);
            self.content.spawn(parent, theme);
            if let Some(bumper) = self.bumper {bumper.spawn(parent, theme);}
        });
    }
}


//if MOBILE && self.navigator { nav_system.tab_navigator(parent, theme); }
// let nav_system = Navigator::new(
//     Some("wordmark".to_string()),
//     Some("Ella Couch".to_string()),
//     vec![Tab{name: "Bitcoin".to_string(), icon: "wallet".to_string(), navigate_to: NavigateTo::Bitcoin, selected: true}],
// );
