use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    DualSwords,
    Fists,
}

#[derive(Serialize, Deserialize)]
pub struct Weapon {
    pub weapon_type: WeaponType
}

// Double swords weapons
pub const TESTING_SWORDS: Weapon = Weapon {
    weapon_type: WeaponType::DualSwords
};

// Fist weapons
pub const BARE_FISTS: Weapon = Weapon {
    weapon_type: WeaponType::Fists
};