use bevy::prelude::*;

use crate::Theme;
use crate::theme::NavigateTo;

use crate::layout::utils::text;
use crate::layout::utils::Size;
use crate::layout::utils::EXPAND;

use crate::components::icon_button::icon_button;

pub struct Header {
    title: String,
    size: Size,
    icon_left: Option<(ImageNode, NavigateTo)>,
    icon_right: Option<(ImageNode, NavigateTo)>,
    profile_photo: bool,
}

impl Header {
    pub fn new(    
        t: &str,
        size: Size,
        icon_left: Option<(ImageNode, NavigateTo)>,
        icon_right: Option<(ImageNode, NavigateTo)>,
        profile_photo: bool,
    ) -> Self {
        Header {title: t.to_string(), size, icon_left, icon_right, profile_photo}
    }
    pub fn spawn_under(&self, parent: &mut ChildBuilder, theme: &Res<Theme>) {
        let font = theme.fonts.style.heading.clone();

        let font_size = match self.size {
            Size::Large => theme.fonts.size.h3,
            Size::Medium => theme.fonts.size.h4,
        };

        parent.spawn(Node{
            width: EXPAND,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(24.0)),
            ..default()
        }).with_children(|parent| {
            icon_button(parent, self.icon_left.clone());
            //pfp_button(parent, self.profile_photo);
            text(parent, &self.title, font, font_size, theme.colors.text.heading);
            icon_button(parent, self.icon_right.clone());
        });
    }
}