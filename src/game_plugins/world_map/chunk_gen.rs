use bevy::prelude::*;
use bevy_ecs_tilemap::{
    map::{TilemapId, TilemapTexture},
    tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex}, TilemapBundle,
};
use noise::{NoiseFn, Perlin};

use crate::consts::{CHUNK_SIZE, NOISE_SCALE, TILE_SIZE, TILE_PIXEL_SIZE};

use super::{utils::{chunks_to_world, world_to_tiles}, world_gen::{TileMap, Tile, RngJesus}};

pub fn spawn_chunks(
    commands: &mut Commands,
    asset_server: &AssetServer,
    rng_jesus: &RngJesus,
    chunk_position: IVec2,
) -> Entity {
    // chunk_manager.spawned_chunks.insert(IVec2::new(x, y));

    let tilemap_entity = commands.spawn_empty().insert(TileMap).id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    let image_handles = vec![
        asset_server.load("tiles/deep-water.png"),
        asset_server.load("tiles/water.png"),
        asset_server.load("tiles/beach.png"),
        asset_server.load("tiles/grass/grass_32x32_0.png"),
    ];
    let texture_vec = TilemapTexture::Vector(image_handles);

    let perlin = Perlin::new(rng_jesus.seed as u32);

    // TODO: Improvement -> player should be in the middle of the chunk not at the bottom
    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };

            // let (world_x, world_y) = chunks_to_world(chunk_position, tile_pos);
            let (perl_x, perl_y) = world_to_tiles(chunks_to_world(chunk_position, tile_pos));

            let perlin_value =
                perlin.get([perl_x as f64 / NOISE_SCALE, perl_y as f64 / NOISE_SCALE]);

            let texture_index: usize = if perlin_value > 0.6 {
                0
            } else if perlin_value > 0.3 {
                1
            } else if perlin_value > 0.2 {
                2
            } else {
                3
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index as u32),
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
        .insert(Tile);

    return tilemap_entity;
}
