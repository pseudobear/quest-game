use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum ConsumableType{}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumable {
    pub consumable_type: ConsumableType,
    pub num_stacked: u8,
}