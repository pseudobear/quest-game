use bevy::prelude::*;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum WeaponType {
    DualSwords,
    Fists,
}

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