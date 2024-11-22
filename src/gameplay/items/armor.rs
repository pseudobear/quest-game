use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum ArmorType {}

#[derive(Serialize, Deserialize)]
pub struct Armor {
    pub armor_type: ArmorType
}