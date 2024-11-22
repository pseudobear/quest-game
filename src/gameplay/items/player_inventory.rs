use crate::gameplay::items::Item;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Resource, Serialize, Deserialize)]
pub struct PlayerInventory {
    inventory: Vec<Item>,
    used_space: usize,
    capacity: usize,
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self {
            inventory: Vec::new(),
            used_space: 0,
            capacity: 128,
        }
    }
}

pub struct PlayerInventoryPlugin;

/// This plugin handles item related stuff, including equip components and inventory resources
/// This module also provides systems for spawning items in the world and displaying them + animations
impl Plugin for PlayerInventoryPlugin {
    fn build(&self, app: &mut App) {
        // TODO: init this with persistent state stored either in file system or in server
        app.init_resource::<PlayerInventory>();
    }
}