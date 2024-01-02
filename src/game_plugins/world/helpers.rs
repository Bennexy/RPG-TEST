use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Res, ResMut},
    math::{IVec2, IVec3, Vec2, Vec3},
    sprite::TextureAtlas, log::{debug, info},
};

use crate::{
    consts::{CHUNK_SIZE, TILE_SIZE},
    game_plugins::world::chunks::ChunkConfig,
};

#[allow(dead_code)]
pub fn ivec3_into_ivec2(ivec3: &IVec3) -> IVec2 {
    return IVec2 {
        x: ivec3.x,
        y: ivec3.y,
    };
}

#[allow(dead_code)]
pub fn pixel_to_tile_pos(position: &Vec3) -> IVec2 {
    let x_res = (position.x) / TILE_SIZE.x;
    let y_res = (position.y) / TILE_SIZE.y;

    let x = if position.x.is_sign_positive() {
        (x_res).ceil() as i32
    } else {
        (x_res).floor() as i32
    };
    let y = if position.y.is_sign_positive() {
        (y_res).ceil() as i32
    } else {
        (y_res).floor() as i32
    };

    return IVec2 { x, y };
}

#[allow(dead_code)]
pub fn pixel_to_chunk_pos(position: &Vec3, config: &ChunkConfig) -> IVec2 {
    let x_res = (position.x) / TILE_SIZE.x / config.chunk_size as f32;
    let y_res = (position.y) / TILE_SIZE.y / config.chunk_size as f32;


    let x = if position.x.is_sign_positive() {
        (x_res).ceil() as i32
    } else {
        (x_res).floor() as i32
    };
    let y = if position.y.is_sign_positive() {
        (y_res).ceil() as i32
    } else {
        (y_res).floor() as i32
    };

    let res = IVec2 { x, y };
    debug!("{x_res}, {y_res} -> {}", res);

    return res;
}

#[allow(dead_code)]
pub fn tile_to_chunk_pos(tile_pos: &IVec2, config: &ChunkConfig) -> IVec2 {
    let x_res = (tile_pos.x as f32) / config.chunk_size as f32;
    let y_res = (tile_pos.y as f32) / config.chunk_size as f32;

    let x = if x_res.is_sign_positive() {
        (x_res).ceil() as i32
    } else {
        (x_res).floor() as i32
    };
    let y = if y_res.is_sign_positive() {
        (y_res).ceil() as i32
    } else {
        (y_res).floor() as i32
    };

    // let x = if x == 0 { 1 } else { x };

    // let y = if y == 0 { 1 } else { y };

    let chunk_pos: IVec2 = IVec2::new(x, y);

    return chunk_pos;
}

#[allow(dead_code)]
pub fn load_texture_atlas_handel(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> bevy::prelude::Handle<TextureAtlas> {
    let texture_handle = asset_server.load("tiles/sprite-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        TILE_SIZE,
        2,
        2,
        Some(Vec2 { x: 2.0, y: 2.0 }),
        None,
    );
    let texture_atlas_handle: bevy::prelude::Handle<TextureAtlas> =
        texture_atlases.add(texture_atlas);

    return texture_atlas_handle;
}
