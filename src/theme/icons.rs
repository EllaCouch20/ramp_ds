use std::collections::BTreeMap;
use bevy::prelude::*;

pub struct IconResources {
    pub icons: BTreeMap<AssetPath, ImageNode>
}

impl IconResources {
    pub fn new(assets: &Res<AssetServer>) -> Self {
        if let Ok(handles) = assets.load_folder("icons") {
            let handles = handles.iter().flat_map(|h| h.try_typed::<Image>().ok()).collect::<Vec<_>>();
            IconResources{icons: BTreeMap::from_iter(handles.into_iter().flat_map(|h| h.path().map(|p| (p, ImageNode::new(h)))))}
        } else {panic!("Icons not loaded");}
    }

    pub fn get(&self, name: &str) -> Option<&ImageNode> {
        self.icons.get(AssetPath::from_str(name))
    }
}