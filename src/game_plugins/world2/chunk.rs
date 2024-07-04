use bevy::{utils::HashMap, math::IVec2};
use bevy::prelude::*;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};


use crate::game_plugins::world::chunks::Tile;


#[derive(Component, Deserialize, Debug)]
pub struct TileEntity {
    tile: Tile,
    entity: Entity,
}

impl Serialize for TileEntity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                let mut te = serializer.serialize_struct("TileEntity", 1).unwrap();

                te.serialize_field("tile", &self.tile).unwrap();
                te.end()
    }
}
impl Deserialize for TileEntity {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        
    }
}

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct TileMap {
    map: HashMap<IVec2, TileEntity>
}

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Chunk {
    position: IVec2,
    tiles: TileMap
}