use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum AccessoryType{}

#[derive(Debug, Serialize, Deserialize)]
pub struct Accessory {
    pub accessory_type: AccessoryType
}