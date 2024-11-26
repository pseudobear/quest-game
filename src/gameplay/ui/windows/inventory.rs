use bevy_egui::{egui, EguiContexts, EguiPlugin};
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

pub fn inventory(mut opened: ResMut<InventoryOpened>, mut contexts: EguiContexts) {
    egui::Window::new("Inventory")
    .open(&mut opened.0)
    .collapsible(false)
    .fixed_size(egui::Vec2::new(INVENTORY_SIZE_X, INVENTORY_SIZE_Y))
    .show(contexts.ctx_mut(), |ui| {
        ui.set_width(ui.available_width());
        ui.set_height(ui.available_height());

        ui.label("world");
        ui.label("world");
        ui.label("world");
        ui.label("world");
    });
}