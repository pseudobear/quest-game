use bevy::prelude::*;


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum MaterialType{}

pub struct Material {
    pub material_type: MaterialType,
    pub num_stacked: u8,
}