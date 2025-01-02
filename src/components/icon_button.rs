use bevy::prelude::*;
use crate::traits::{Component};
use crate::theme::NavigateTo;
use crate::Theme;

pub struct IconButton(pub Option<(ImageNode, NavigateTo)>);

impl Component for IconButton {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder<'_>, theme: &Res<Theme>) {
        let node = Node {
            height: Val::Px(32.0),
            width: Val::Px(32.0),
            ..default()
        };
        if let Some(bundle) = self.0 {
            parent.spawn((node, bundle));
        } else {
            parent.spawn(node);
        }
    }
}
