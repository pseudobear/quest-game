use bevy::prelude::*;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum ConsumableType{}

pub struct Consumable {
    pub consumable_type: ConsumableType,
    pub num_stacked: u8,
}