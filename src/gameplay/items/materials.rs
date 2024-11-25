use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, Eq, PartialEq, Debug, Hash, Serialize, Deserialize)]
pub enum MaterialType{}

#[derive(Debug, Serialize, Deserialize)]
pub struct Material {
    pub material_type: MaterialType,
    pub num_stacked: u8,
}