use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, transform::commands,
    utils::hashbrown::HashMap, window::close_on_esc,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bincode;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Write},
    time::Instant,
};

mod game_plugins;
// mod pig;
mod ui;
mod zoom;
// local uses
use game_plugins::{
    player::{Player, PlayerPlugin},
    tree::TreePlugin,
    world::{chunk::Chunk, tile::Tile},
    // world_map::world_gen::WorldGenPlugin,
};

use rand::{thread_rng, Rng};
use ui::GameUI;
use zoom::ScaleableWorldViewPlugin;

use crate::consts::TILE_SIZE;

mod consts;

fn main() {
    App::new()
        .init_resource::<ChunkManger>()
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "AutoRPG".into(),
                        resolution: (1280.0, 620.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F2)),
        )
        .add_plugins((
            ScaleableWorldViewPlugin,
            TreePlugin,
            PlayerPlugin,
            GameUI,
            // WorldGenPlugin,
        ))
        .add_systems(
            Update,
            (
                close_on_esc,
                print_player_chunk_pos, // regenerate,
                                        // spawn_chunks_around_camera2,
                                        // spawn_chunks_around_camera
                                        // change_world_scale,
                                        // character_movement
                                        //     despawn_outofrange_chunks,
                                        // save_drawn_chunks
            ),
        )
        .add_systems(Startup, spawn_chunk)
        .run();
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    player_pos: Query<(&mut Transform, Entity), With<Player>>,
    chunk_manager: Res<ChunkManger>,
) {
    let _ = player_pos.single();
    info!("got chunks from main");
}

pub trait Renderable {
    fn render(
        &self,
        commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlas: ResMut<Assets<TextureAtlas>>,
    );
}

pub struct ChunkConfig {
    chunk_size: u16,
    render_size: u16,
}

impl Default for ChunkConfig {
    fn default() -> Self {
        return Self {
            chunk_size: 1,
            render_size: 2,
        };
    }
}

#[derive(Resource)]
pub struct ChunkManger {
    chunks: HashMap<(i32, i32), Chunk>,
    config: ChunkConfig,
}

impl Default for ChunkManger {
    fn default() -> Self {
        return Self::new();
    }
}

impl ChunkManger {
    fn new() -> Self {
        let config = ChunkConfig::default();
        let mut chunks = HashMap::new();

        let mut spite = 0;
        for x in -(config.render_size as i32)..=(config.render_size as i32) {
            for y in -(config.render_size as i32)..=(config.render_size as i32) {
                spite += 1;
                if spite >= 4 {
                    spite = 0;
                }
                chunks.insert(
                    (x, y),
                    Chunk::generate(Some(config.chunk_size), Some(spite)),
                );
            }
        }

        return Self { chunks, config };
    }

    fn get_drawable_chunk_data(&self, chunk_position: &(i32, i32)) -> Chunk {
        let mut drawable_chunk = Chunk::empty();

        let tile_size: Vec3 = Vec3 {
            x: TILE_SIZE.x,
            y: TILE_SIZE.y,
            z: 1.0,
        };
        let chunk_pos = Vec3 {
            x: (chunk_position.0 * (self.config.chunk_size * 2 + 1) as i32) as f32,
            y: (chunk_position.1 * (self.config.chunk_size * 2 + 1) as i32) as f32,
            z: 0.0,
        };

        let chunk2draw = self.chunks.get(chunk_position).unwrap();
        for tile in chunk2draw.0.values() {
            let drawable_grid_position = (chunk_pos + tile.grid_position) * tile_size * tile.scale;
            drawable_chunk.insert_tile(Tile {
                grid_position: drawable_grid_position,
                scale: tile.scale,
                spite_index: tile.spite_index,
            })
        }

        return drawable_chunk;
    }

    fn player_tile_position(&self, player_pos: &Transform) -> (i32, i32) {
        let tile_size: Vec3 = Vec3 {
            x: TILE_SIZE.x,
            y: TILE_SIZE.y,
            z: 1.0,
        };

        let tile_offset = Vec3 {
            x: -(TILE_SIZE.x / 2.),
            y: -(TILE_SIZE.y / 2.),
            z: 0.0,
        };
        let chunk_pos = (player_pos.translation - tile_offset) / tile_size;

        return (chunk_pos.x.floor() as i32, chunk_pos.y.floor() as i32);
    }

    // TODO: improve function -> it is a mess.
    fn player_chunk_postion(&self, player_pos: &Transform) -> (i32, i32) {
        let tile_pos = self.player_tile_position(player_pos);


        let offset_tile_x = if tile_pos.0 == 0 {
            0
        } else if tile_pos.0.is_positive() {
            if tile_pos.0 <= self.config.chunk_size as i32 {0} else {tile_pos.0 - self.config.chunk_size as i32}
        } else {
            if tile_pos.0 >= -(self.config.chunk_size as i32) {0} else {tile_pos.0 + self.config.chunk_size as i32}
        };

        let offset_tile_y = if tile_pos.1 == 0 {
            0
        } else if tile_pos.1.is_positive() {
            if tile_pos.1 <= self.config.chunk_size as i32 {0} else {tile_pos.1 - self.config.chunk_size as i32}
        } else {
            if tile_pos.1 >= -(self.config.chunk_size as i32) {0} else {tile_pos.1 + self.config.chunk_size as i32}
        };

        let chunk_pos = (
            if offset_tile_x == 0 {0.} else {(offset_tile_x as f32 / (self.config.chunk_size * 2 +1) as f32 )as f32},
            if offset_tile_y == 0 {0.} else {(offset_tile_y as f32 / (self.config.chunk_size * 2 +1) as f32 )as f32},
        );

        let chunk_x = if chunk_pos.0.is_sign_positive() {
            chunk_pos.0.ceil()
        } else {
            chunk_pos.0.floor()
        };

        let chunk_y = if chunk_pos.1.is_sign_positive() {
            chunk_pos.1.ceil()
        } else {
            chunk_pos.1.floor()
        };

        let chunk_pos = (chunk_x as i32, chunk_y as i32);

        // info!("tile_pos {:?}, offset_tile_pos: {:?} chunk_pos: {:?}", tile_pos, (offset_tile_x, offset_tile_y), chunk_pos);

        return chunk_pos
    }
}

fn spawn_chunk(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    chunk_manager: Res<ChunkManger>,
) {
    // let image = asset_server.load("tiles/grass/grass_32x32_0.png");
    let texture_handle = asset_server.load("tiles/sprite-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32.0, 32.0),
        2,
        2,
        Some(Vec2 { x: 2.0, y: 2.0 }),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    debug!("drawing {} chunks", chunk_manager.chunks.len());
    for (chunk_pos, _chunk) in &chunk_manager.chunks {
        let chunk = chunk_manager.get_drawable_chunk_data(chunk_pos);
        for tile in chunk.0.values() {
            commands.spawn((
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(tile.spite_index),
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: tile.into(),
                    ..default()
                },
                *tile,
            ));
        }
    }
}

fn print_player_chunk_pos(
    player_pos: Query<&Transform, With<Player>>,
    chunk_manager: Res<ChunkManger>,
) {
    let chunks = chunk_manager.player_chunk_postion(player_pos.single());

    info!("player chunks: {:?}", chunks);
}
