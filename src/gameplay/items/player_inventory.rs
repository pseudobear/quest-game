use crate::gameplay::items::Item;
use crate::gameplay::GameState;
use bevy_pkv::PkvStore;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use blake2::{Blake2s256, Digest};


#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct PlayerInventory {
    inventory: Vec<Item>,
    used_space: usize,
    capacity: usize,
}

impl PlayerInventory {

    // serialize to a sequence of bytes in CBOR format.
    fn serialize(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self).unwrap()
    }

    // deserialize a sequence of bytes into a PlayerInventory struct. The byte
    fn deserialize_bytes(byte_vec: &Vec<u8>) -> Result<PlayerInventory, serde_cbor::Error> {
        serde_cbor::from_slice(byte_vec)
    }

    // deserialize a sequence of bytes into a PlayerInventory struct. The byte
    fn deserialize_hex(hex: &str) -> Result<PlayerInventory, serde_cbor::Error> {
        let byte_vec = base16ct::lower::decode_vec(hex);
        serde_cbor::from_slice(&byte_vec.unwrap())
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self {
            inventory: Vec::new(),
            used_space: 0,
            capacity: 128,
        }
    }
}

pub struct PlayerInventoryPlugin;

/// This plugin handles item related stuff, including equip components and inventory resources
/// This module also provides systems for spawning items in the world and displaying them + animations
impl Plugin for PlayerInventoryPlugin {
    fn build(&self, app: &mut App) {
        // TODO: init this with persistent state stored either in file system or in server
        app.init_resource::<PlayerInventory>()
           .add_systems(Startup, initialize_inventory)
           .add_systems(Update, store_inventory_to_pkv.run_if(in_state(GameState::Playing)));
    }
}

fn initialize_inventory(
    pkv: ResMut<PkvStore>,
    mut player_inventory: ResMut<PlayerInventory>
) {
    *player_inventory = PlayerInventory::deserialize_hex(
        &pkv.get::<String>("PlayerInventory").unwrap()
    ).unwrap_or_default();
}

fn store_inventory_to_pkv(
    mut pkv: ResMut<PkvStore>,
    player_inventory: Res<PlayerInventory>
) {
    if player_inventory.is_changed() && !player_inventory.is_added() {
        let serialized_inventory = player_inventory.serialize();

        // compute hash of serialized inventory for integrity checking
        // TODO: Move this to serverside and has with a secret (for integrity checking)
        let mut hasher = Blake2s256::new();
        hasher.update(&serialized_inventory);
        let hash = hasher.finalize();

        pkv.set::<String>("PlayerInventory", &base16ct::lower::encode_string(&serialized_inventory))
            .expect("Failed to store PlayerInventory");
        pkv.set::<String>("PlayerInventoryHash", &base16ct::lower::encode_string(&hash))
            .expect("Failed to store PlayerInventory");

        /* debugging prints
        println!("{:?}", pkv.get::<String>("PlayerInventory").unwrap());
        println!("{:?}", PlayerInventory::deserialize_hex(&pkv.get::<String>("PlayerInventory").unwrap()).unwrap());
        */
    }
}