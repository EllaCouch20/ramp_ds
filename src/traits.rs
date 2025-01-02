use bevy::prelude::*;
use crate::Theme;


pub trait Parent {
    fn spawn<'c>(&'c mut self, bundle: impl Bundle) -> EntityCommands<'c> where Self: Sized;
}

impl Parent for Commands<'_, '_> {
    fn spawn<'c>(&'c mut self, bundle: impl Bundle) -> EntityCommands<'c> where Self: Sized {
        Commands::spawn(self, bundle)
    }
}

impl Parent for ChildBuilder<'_> {
    fn spawn<'c>(&'c mut self, bundle: impl Bundle) -> EntityCommands<'c> where Self: Sized {
        ChildBuild::spawn(self, bundle)
    }
}

pub trait Component {
    fn spawn(self, parent: &mut impl Parent, theme: &Res<Theme>);
}