use cosmwasm_std::Addr;
use schemars::JsonSchema;
use secret_toolkit::storage::{Keymap, KeymapBuilder};
use secret_toolkit::{serialization::Bincode2, storage::Item};
use serde::{Deserialize, Serialize};

pub const TABLE_INFO: Item<TableInfo> = Item::new(b"contacts_count");

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct TableInfo {
    owner: Addr,        // owner of table
    table_name: String, // custom table name
    table_index: u32,
}

pub const TABLE: Keymap<CellCoords, String, Bincode2> = KeymapBuilder::new(b"secrets").build();
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct CellCoords {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Cell {
    coords: CellCoords,
    value: String,
}

// TODO: functions to interact with Map in a friendly way
