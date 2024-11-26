use crate::ui::windows::spawn_ui_window;
use bevy::prelude::*;


const INVENTORY_SIZE_X: f32 = 200.;
const INVENTORY_SIZE_Y: f32 = 100.;

#[derive(Component, Default)]
pub struct InventoryWindow;

pub fn open_inventory(
    mut commands: Commands,
    inventory_query: Query<&InventoryWindow>
) {
    if inventory_query.is_empty() {
        spawn_ui_window::<InventoryWindow>(
            &mut commands,
            Vec2::new(INVENTORY_SIZE_X, INVENTORY_SIZE_Y),
            "Inventory"
        );
    }
}