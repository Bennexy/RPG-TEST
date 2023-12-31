use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::close_on_esc, utils::hashbrown::HashMap, transform::commands};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use serde::{Deserialize, Serialize};
use bincode;
use std::{io::{BufWriter, BufReader, Write}, fs::{File, self}, time::Instant};

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
                regenerate,
                // spawn_chunks_around_camera2,
                // spawn_chunks_around_camera
                // change_world_scale,
                // character_movement
                //     despawn_outofrange_chunks,
                save_drawn_chunks
            ),
        )
        .add_systems(Startup, draw_chunk)
        .run();
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    player_pos: Query<(&mut Transform, Entity), With<Player>>,
    chunk_manager: Res<ChunkManger>
) {
    let _ = player_pos.single();
    info!("got chunks from main");
}


#[derive(Resource)]
pub struct ChunkManger {
    chunks: HashMap<(i32, i32), Chunk>
}

impl Default for ChunkManger {
    fn default() -> Self {
        return Self::new();
    }
}

impl ChunkManger {
    fn new() -> Self {
        let mut chunks = HashMap::new();

        for x in -1..=1 {
            for y in -1..1 {
                chunks.insert((x,y), Chunk::generate(None, None));
            }
        }

        return Self { chunks }
    }
}


fn spawn_chunk(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // let image = asset_server.load("tiles/grass/grass_32x32_0.png");
    let texture_handle = asset_server.load("tiles/sprite-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 2, Some(Vec2 { x: 2.0, y: 0.0 }), None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let chunk = Chunk::generate(None, None);
    
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


fn draw_chunk(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("tiles/sprite-sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 2, 2, Some(Vec2 { x: 2.0, y: 0.0 }), None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let chunk = match Chunk::load_from_file("chunk.bin") {
        Some(val) => val,
        None => Chunk::generate(None, None)
    };

    let start = Instant::now();
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
    let end = start.elapsed();


    info!("drawing chunk took: {} seconds", end.as_secs_f64());
}




fn regenerate(
    commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    input: Res<Input<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::Tab) {
        return
    }
    if input.pressed(KeyCode::ShiftLeft) {
        draw_chunk(commands, asset_server, texture_atlases);
        return;
    }

    spawn_chunk(commands, asset_server, texture_atlases);

}

fn save_drawn_chunks(
    input: Res<Input<KeyCode>>,
    tiles: Query<&Tile, With<Tile>>
) {
    if !input.pressed(KeyCode::ControlLeft) {
        return;
    }

    if !input.just_pressed(KeyCode::X) {
        return;
    }

    info!("tiles to save: {}", tiles.iter().len());

    let mut chunk = Chunk::empty();
    tiles.for_each(|tile| {chunk.insert_tile(*tile)});

    chunk.save_to_file();
}