pub mod weapons;

use crate::gameplay::items::weapons::Weapon;
use bevy::prelude::*;


pub struct ItemsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct CharacterEquips {
    pub weapon: Weapon,
}