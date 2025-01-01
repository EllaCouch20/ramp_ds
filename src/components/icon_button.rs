use bevy::prelude::*;
use crate::theme::NavigateTo;

pub fn icon_button(
    parent: &mut ChildBuilder,
    icon_data: Option<(ImageNode, NavigateTo)>,
){
    let mut child = parent.spawn(Node {
        height: Val::Px(32.0),
        width: Val::Px(32.0),
        ..default()
    });

    if let Some((icon, navigate_to)) = icon_data {
        child.insert(navigate_to);
        child.insert(icon);
    }
}
