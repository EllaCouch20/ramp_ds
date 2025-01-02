use std::collections::BTreeMap;
use bevy::prelude::*;

#[derive(Clone)]
pub struct IconResources {
    pub icons: BTreeMap<String, Handle<Image>>
}

impl IconResources {
    pub fn new(asset_server: Res<AssetServer>, icons: Vec<String>) -> Self {
       IconResources{icons: BTreeMap::from_iter(icons.into_iter().map(|p| (p.clone(), asset_server.load(format!("icons/{p}.png")))))}
    }

    pub fn get(&self, name: &str) -> ImageNode {
        ImageNode::new(self.icons.get(name).cloned().unwrap())
    }
}

#[derive(Clone)]
pub struct Icons {
    pub vec: Vec<String>
}

impl Default for Icons {
    fn default() -> Self {
        Icons {
            vec: vec![
                "file".to_string(),
                "folder".to_string(),
                "delete".to_string(),
                "save".to_string(),
                "exit".to_string()
            ]
        }
    }
}