use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapTexture},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
    TilemapBundle,
};
use noise::{NoiseFn, Perlin};
use strum::EnumCount;
use strum_macros::EnumCount as EnumCountMacro;

use crate::consts::{CHUNK_SIZE, NOISE_SCALE, TILE_PIXEL_SIZE, TILE_SIZE};

use super::{
    utils::{chunks_to_world, world_to_tiles},
    world_gen::{RngJesus, TileMap, Chunk},
};

pub struct ImageHandles {
    assets: [Handle<Image>; TileType::COUNT],
}

impl ImageHandles {
    fn load(asset_server: &AssetServer) -> Self {
        let assets = [
            asset_server.load("tiles/grass/grass_32x32_0.png"),
            asset_server.load("tiles/beach.png"),
            asset_server.load("tiles/water.png"),
            asset_server.load("tiles/deep-water.png"),
            asset_server.load("tiles/white.png"),
        ];

        return Self { assets };
    }

    fn get_image_index_from_tile_type(tile_type: &TileType) -> u32 {
        match tile_type {
            TileType::GrassLand => 0,
            TileType::Beach => 1,
            TileType::ShallowWater => 2,
            TileType::DeepWater => 3,
            TileType::White => 4,
        }
    }

    fn get_tile_texture_index_from_tile_type(tile_type: &TileType) -> TileTextureIndex {
        let index = ImageHandles::get_image_index_from_tile_type(tile_type);

        TileTextureIndex(index)
    }
}

impl From<TileType> for TileTextureIndex {
    fn from(tile_type: TileType) -> Self {
        ImageHandles::get_tile_texture_index_from_tile_type(&tile_type)
    }
}

#[derive(Debug, EnumCountMacro)]
pub enum TileType {
    GrassLand,
    Beach,
    ShallowWater,
    DeepWater,
    White,
}

struct Wrapper<T>(Vec<T>);
impl<T> From<Wrapper<T>> for Vec<T> {
    fn from(w: Wrapper<T>) -> Vec<T> {
        w.0
    }
}

#[derive(Debug, EnumCountMacro)]
pub enum BiomType {
    // Moutains,
    GrassLand,
    Ocean,
    // Islands,
}

pub trait BiomTiles {
    fn get_biom(&self, chunk_position: &IVec2) -> BiomType;
    fn get_tile_type(&self, biom: &BiomType, ttile_pos: &IVec2) -> TileType;
}

impl BiomTiles for RngJesus {
    fn get_biom(&self, chunk_position: &IVec2) -> BiomType {
        let perlin = Perlin::new(self.biom_seed);
        let value = perlin.get([
            chunk_position.x as f64 / (NOISE_SCALE), // * CHUNK_SIZE.x as f64),
            chunk_position.y as f64 / (NOISE_SCALE), // * CHUNK_SIZE.y as f64),
        ]);

        if value > -0.3 {
            return BiomType::GrassLand;
        } else {
            return BiomType::Ocean;
        }

        // if value > 0.75 {
        //     return BiomType::Moutains;
        // } else if value > -0.2 {
        //     return BiomType::GrassLand;
        // } else if value > -0.5 {
        //     return BiomType::Islands;
        // } else {
        //     return BiomType::Ocean;
        // }
    }

    fn get_tile_type(&self, biom: &BiomType, tile_pos: &IVec2) -> TileType {
        let _ = biom;
        let biom_perlin = Perlin::new(self.biom_seed);
        let perlin1 = Perlin::new(self.seed);

        let biom_point = [
            tile_pos.x as f64 / (NOISE_SCALE * CHUNK_SIZE.x as f64),
            tile_pos.y as f64 / (NOISE_SCALE * CHUNK_SIZE.y as f64),
        ];

        let tile_point = [
            tile_pos.x as f64 / (NOISE_SCALE * 2 as f64),
            tile_pos.y as f64 / (NOISE_SCALE * 2 as f64),
        ];

        let biom_perlin_value = biom_perlin.get(biom_point);
        let tile_perlin_value = perlin1.get(tile_point);

        // let biom_perlin_value = if biom_perlin_value > 0.9 {
        //     biom_perlin_value * 2.5
        // } else if biom_perlin_value > 0.85 {
        //     biom_perlin_value * 2.25
        // } else if biom_perlin_value > 0.8 {
        //     biom_perlin_value * 2.0
        // } else if biom_perlin_value > 0.75 {
        //     biom_perlin_value * 1.75
        // } else if biom_perlin_value > 0.7 {
        //     biom_perlin_value * 1.5
        // } else if biom_perlin_value > 0.65 {
        //     biom_perlin_value * 1.25
        // } else if biom_perlin_value < -0.9 {
        //     biom_perlin_value * 3.25
        // } else if biom_perlin_value < -0.85 {
        //     biom_perlin_value * 3.0
        // } else if biom_perlin_value < -0.8 {
        //     biom_perlin_value * 2.75
        // } else if biom_perlin_value < -0.75 {
        //     biom_perlin_value * 2.5
        // } else if biom_perlin_value < -0.7 {
        //     biom_perlin_value * 2.25
        // } else if biom_perlin_value < -0.65 {
        //     biom_perlin_value * 2.0
        // } else if biom_perlin_value < -0.5 {
        //     biom_perlin_value * 1.75
        // } else if biom_perlin_value < -0.45 {
        //     biom_perlin_value * 1.5
        // } else if biom_perlin_value < -0.4 {
        //     biom_perlin_value * 1.25
        // } else {
        //     biom_perlin_value
        // };

        let tile_perlin_value = if biom_perlin_value.abs() > 0.3 {
            tile_perlin_value
        } else if biom_perlin_value.abs() > 0.25 {
            tile_perlin_value / 1.125
        } else if biom_perlin_value.abs() > 0.2 {
            tile_perlin_value / 1.25
        } else if biom_perlin_value.abs() > 0.15 {
            tile_perlin_value / 1.375
        } else if biom_perlin_value.abs() > 0.1 {
            tile_perlin_value / 1.5
        } else {tile_perlin_value / 1.625};

        let biom_perlin_value = if biom_perlin_value.abs() > 0.8 {
            biom_perlin_value * 2.5
        } else if biom_perlin_value.abs() > 0.75 {
            biom_perlin_value * 2.25
        } else if biom_perlin_value.abs() > 0.7 {
            biom_perlin_value * 2.0
        } else if biom_perlin_value.abs() > 0.65 {
            biom_perlin_value * 1.75
        } else if biom_perlin_value.abs() > 0.6 {
            biom_perlin_value * 1.5
        } else if biom_perlin_value.abs() > 0.55 {
            biom_perlin_value * 1.25
        } else {biom_perlin_value};
        
         if tile_perlin_value + biom_perlin_value > -0.45 {
            TileType::GrassLand
        } else if tile_perlin_value + biom_perlin_value > -0.9 {
            TileType::Beach
        } else if tile_perlin_value + biom_perlin_value > -1.6 {
            TileType::ShallowWater
        } else {
            TileType::DeepWater
        }
        // if tile_perlin_value + biom_perlin_value > 1.3 {
        //     TileType::White
        // } else if tile_perlin_value + biom_perlin_value > -0.55 {
        //     TileType::GrassLand
        // } else if tile_perlin_value + biom_perlin_value > -0.7 {
        //     TileType::Beach
        // } else if tile_perlin_value + biom_perlin_value > -1.1 {
        //     TileType::ShallowWater
        // } else {
        //     TileType::DeepWater
        // }


        // match biom {
        //     // BiomType::Moutains => TileType::White,
        //     BiomType::GrassLand => {
        //         if tile_perlin_value - biom_perlin_value > 0.9 {
        //             TileType::White
        //         } else if tile_perlin_value - biom_perlin_value > -0.6 {
        //             TileType::GrassLand
        //         } else if tile_perlin_value - biom_perlin_value > -0.7 {
        //             TileType::Beach
        //         } else if tile_perlin_value - biom_perlin_value > -0.95 {
        //             TileType::ShallowWater
        //         } else {
        //             TileType::DeepWater
        //         }
        //     }
        //     BiomType::Ocean => {
        //         if tile_perlin_value > 0.75 {
        //             TileType::Beach
        //         } else if tile_perlin_value > 0.45 {
        //             TileType::ShallowWater
        //         } else {
        //             TileType::DeepWater
        //         }
        //     } // BiomType::Islands => {
        //       //     if perlin_value > 0.5 {
        //       //         return TileType::GrassLand;
        //       //     } else if perlin_value > 0.15 {
        //       //         return TileType::Beach;
        //       //     } else if perlin_value > -0.5 {
        //       //         return TileType::ShallowWater;
        //       //     } else {
        //       //         return TileType::DeepWater;
        //       //     }
        //       // }
        // }
    }
}

pub fn spawn_chunks(
    commands: &mut Commands,
    asset_server: &AssetServer,
    rng_jesus: &RngJesus,
    chunk_position: IVec2,
) -> Entity {
    // chunk_manager.spawned_chunks.insert(IVec2::new(x, y));

    let tilemap_entity = commands.spawn_empty().insert(TileMap).id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    let image_handles = ImageHandles::load(asset_server);
    let texture_vec = TilemapTexture::Vector(image_handles.assets.into());

    let biom = rng_jesus.get_biom(&chunk_position);

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };

            let (tile_x, tile_y) = world_to_tiles(chunks_to_world(chunk_position, tile_pos));
            let global_tile_pos: IVec2 = IVec2 {
                x: tile_x,
                y: tile_y,
            };

            let texture_index = rng_jesus.get_tile_type(&biom, &global_tile_pos);

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: texture_index.into(),
                    // color: color,
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let (x, y) = chunks_to_world(chunk_position, TilePos { x: 0, y: 0 });
    let transform = Transform::from_translation(Vec3::new(x, y, -10.0));
    commands
        .entity(tilemap_entity)
        .insert(TilemapBundle {
            grid_size: TILE_SIZE.into(),
            size: CHUNK_SIZE.into(),
            storage: tile_storage,
            texture: texture_vec,
            tile_size: TILE_PIXEL_SIZE,
            transform: transform,
            ..Default::default()
        })
        .insert(Chunk);

    return tilemap_entity;
}
