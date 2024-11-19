pub mod weapons;
pub mod armor;
pub mod accessories;
pub mod consumables;
pub mod materials;
pub mod player_inventory;

use crate::gameplay::items::weapons::Weapon;
use crate::gameplay::items::armor::Armor;
use crate::gameplay::items::accessories::Accessory;
use crate::gameplay::items::consumables::Consumable;
use crate::gameplay::items::materials::Material;
use crate::gameplay::items::player_inventory::PlayerInventoryPlugin;
use bevy::prelude::*;


pub struct ItemsPlugin;

/// This plugin handles item related stuff, including equip components and inventory resources
/// This module also provides systems for spawning items in the world and displaying them + animations
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerInventoryPlugin,
        ));
    }
}

pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Accessory(Accessory),
    Consumable(Consumable),
    Material(Material),
}

#[derive(Component)]
pub struct CharacterEquips {
    pub weapon: Weapon,
}