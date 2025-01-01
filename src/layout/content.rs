use bevy::prelude::*;

use bevy::prelude::*;

pub struct Content {
    pub alignment: JustifyContent,
    children: Vec<Box<dyn FnOnce(&mut ChildBuilder) + Send + Sync>>,
}

impl Content {
    pub fn new(alignment: JustifyContent) -> Self {
        Self {
            alignment,
            children: Vec::new(),
        }
    }

    pub fn add_content<F>(&mut self, child: F)
    where
        F: FnOnce(&mut ChildBuilder) + 'static + Send + Sync,
    {
        self.children.push(Box::new(child));
    }

    pub fn spawn_under(&mut self, parent: &mut ChildBuilder) {
        parent.spawn(Node {
            justify_content: self.alignment,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            max_width: Val::Px(512.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(24.0),
            padding: UiRect::all(Val::Px(24.0)),
            ..default()
        }).with_children(|builder| {
            for child in self.children.drain(..) {
                child(builder);
            }
        });
    }
}
