use bevy::prelude::*;
use crate::Theme;

pub trait Component {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder<'_>, theme: &Res<Theme>);

    fn box_spawn(self, parent: &mut ChildBuilder<'_>, theme: &Res<Theme>) where Self: Sized {
        Box::new(self).spawn(parent, theme)
    }
}

impl<T: Component> Component for Box<T> {
    fn spawn(self: Box<Self>, parent: &mut ChildBuilder<'_>, theme: &Res<Theme>) {
        (*self).spawn(parent, theme)
    }
}
