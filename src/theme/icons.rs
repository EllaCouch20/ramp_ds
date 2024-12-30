use std::collections::BTreeMap;
use bevy::prelude::*;
use bevy::asset::LoadedFolder;

pub struct IconResources {
    pub icons: BTreeMap<String, ImageNode>
}

impl IconResources {
    pub fn new(asset_server: &Res<AssetServer>, assets: Res<Assets<LoadedFolder>>) -> Self {
        let handle: Handle<LoadedFolder> = asset_server.load_folder("assets/icons");
        let handles: Vec<Handle<Image>> = assets.get(handle.id()).unwrap().handles.clone().into_iter().flat_map(|h| h.try_typed::<Image>().ok()).collect();
        IconResources{icons: BTreeMap::from_iter(handles.into_iter().flat_map(|h| h.path().map(|p| p.to_string()).map(|p| (p, ImageNode::new(h)))))}
    }

    pub fn get(&self, name: &str) -> Option<ImageNode> {
        self.icons.get(&format!("{}.png", name)).cloned()
    }
}
