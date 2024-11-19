use bevy::prelude::*;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum ArmorType {}

pub struct Armor {
    pub armor_type: ArmorType
}