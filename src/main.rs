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
                // print_player_chunk_pos,
                // regenerate,
                // spawn_chunks_around_camera2,
                spawn_chunks_around_camera,
                despawn_chunks_around_camera,
                // change_world_scale,
                // character_movement
                //     despawn_outofrange_chunks,
                // save_drawn_chunks
            ),
        )
        .add_systems(Startup, spawn_chunk)
        .run();
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
            chunk_size: 2,
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
        self.player_chunk_postion_from_tile_position(&tile_pos)
    }

    fn player_chunk_postion_from_tile_position(&self, tile_pos: &(i32, i32)) -> (i32, i32) {
        let offset_tile_x = if tile_pos.0 == 0 {
            0
        } else if tile_pos.0.is_positive() {
            if tile_pos.0 <= self.config.chunk_size as i32 {
                0
            } else {
                tile_pos.0 - self.config.chunk_size as i32
            }
        } else {
            if tile_pos.0 >= -(self.config.chunk_size as i32) {
                0
            } else {
                tile_pos.0 + self.config.chunk_size as i32
            }
        };

        let offset_tile_y = if tile_pos.1 == 0 {
            0
        } else if tile_pos.1.is_positive() {
            if tile_pos.1 <= self.config.chunk_size as i32 {
                0
            } else {
                tile_pos.1 - self.config.chunk_size as i32
            }
        } else {
            if tile_pos.1 >= -(self.config.chunk_size as i32) {
                0
            } else {
                tile_pos.1 + self.config.chunk_size as i32
            }
        };

        let chunk_pos = (
            if offset_tile_x == 0 {
                0.
            } else {
                (offset_tile_x as f32 / (self.config.chunk_size * 2 + 1) as f32) as f32
            },
            if offset_tile_y == 0 {
                0.
            } else {
                (offset_tile_y as f32 / (self.config.chunk_size * 2 + 1) as f32) as f32
            },
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

        return chunk_pos;
    }

    fn save_map(&self) {
        let start = Instant::now();
        let folder = "save/";
        for (chunk_pos, chunk) in &self.chunks {
            chunk.save_to_file(
                format!("{}chunk_{}_{}.bin", folder, chunk_pos.0, chunk_pos.1).as_str(),
            );
        }

        let end = start.elapsed();
        info!(
            "saving all {} chunks took: {} seconds",
            (self.config.render_size * 2 + 1).pow(2),
            end.as_secs_f64()
        );
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
    // chunk_manager.save_map();
}

fn print_player_chunk_pos(
    player_pos: Query<&Transform, With<Player>>,
    chunk_manager: Res<ChunkManger>,
) {
    let chunks = chunk_manager.player_chunk_postion(player_pos.single());

    info!("player chunks: {:?}", chunks);
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    player_pos: Query<&Transform, With<Player>>,
    mut chunk_manager: ResMut<ChunkManger>,
) {
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

    let player_chunk_cords = chunk_manager.player_chunk_postion(&player_pos.single());

    let render_distance_from_player = chunk_manager.config.chunk_size as i32;

    let mut count = 0;
    for x in player_chunk_cords.0 - render_distance_from_player
        ..=player_chunk_cords.0 + render_distance_from_player
    {
        for y in player_chunk_cords.1 - render_distance_from_player
            ..=player_chunk_cords.1 + render_distance_from_player
        {
            if chunk_manager.chunks.get(&(x, y)).is_none() {
                info!("generate new chunk at pos: {:?}", (x, y));
                count += 1;
                if count >= 4 {
                    count = 0
                }
                let chunk = Chunk::generate(Some(chunk_manager.config.chunk_size), Some(count));
                chunk_manager.chunks.insert((x, y), chunk);
                let chunk = chunk_manager.get_drawable_chunk_data(&(x, y));
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
            };
        }
    }
}

fn despawn_chunks_around_camera(
    commands: Commands,
    player_pos: Query<&Transform, With<Player>>,
    mut chunk_manager: ResMut<ChunkManger>,
    tiles: Query<(Entity, &Tile), With<Tile>>,
) {
    let player_chunk_cords = chunk_manager.player_chunk_postion(&player_pos.single());
    let render_distance_from_player = chunk_manager.config.chunk_size as i32;

    let chunks_to_render: Vec<(i32, i32)> = (player_chunk_cords.0 - render_distance_from_player
        ..=player_chunk_cords.0 + render_distance_from_player)
        .flat_map(|x| {
            (player_chunk_cords.1 - render_distance_from_player
                ..=player_chunk_cords.1 + render_distance_from_player)
                .map(move |y| (x, y))
                .collect::<Vec<(i32, i32)>>()
        })
        .collect();

    for (entity, tile) in tiles.iter() {
        let tile_chunk = &chunk_manager.player_chunk_postion_from_tile_position(&(
            tile.grid_position.x as i32,
            tile.grid_position.y as i32,
        ));
        if !chunks_to_render.contains(&tile_chunk) {}
        info!("found tile in vec {:?}", tile_chunk);
    }
}
