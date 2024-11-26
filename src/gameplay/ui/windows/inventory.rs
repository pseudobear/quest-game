use bevy::prelude::*;


const INVENTORY_SIZE_X: f32 = 500.;
const INVENTORY_SIZE_Y: f32 = 250.;

#[derive(Resource)]
pub struct InventoryOpened(bool);

impl FromWorld for InventoryOpened {
    fn from_world(_world: &mut World) -> Self {
        InventoryOpened(false)
    }
}

pub fn open_inventory(mut opened: ResMut<InventoryOpened>) {
    println!("TRIGGERED OPEN INVENTORY");

    opened.0 = true;
}

pub fn inventory(mut opened: ResMut<InventoryOpened>) {
}