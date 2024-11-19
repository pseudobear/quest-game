use bevy::prelude::*;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AccessoryType{}

pub struct Accessory {
    pub accessory_type: AccessoryType 
}