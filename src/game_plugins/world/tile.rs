use std::{ops::Range, time::Instant};

use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::{
    consts::TILE_SIZE,
    game_plugins::{
        player::Player,
        world::helpers::{pixel_to_chunk_pos, tile_to_chunk_pos},
    },
};

#[derive(Component)]
pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileManager>()
            .add_event::<PlayerTileChangeEvent>()
            .add_systems(Startup, spawn_chunk)
            .add_systems(
                Update,
                (
                    detect_player_movement.before(render_chunks_around_player),
                    render_chunks_around_player,
                ),
            );
    }
}

#[derive(Component, Serialize, Deserialize, Debug)]
pub struct TileConfig {
    render_distance: u32,
}

impl Default for TileConfig {
    fn default() -> Self {
        return Self {
            render_distance: 128,
        };
    }
}

pub fn ivec3_into_ivec2(ivec3: &IVec3) -> IVec2 {
    return IVec2 {
        x: ivec3.x,
        y: ivec3.y,
    };
}

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

    let tile_pos = IVec2 { x, y };

    return tile_pos;
}

#[derive(Component, Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Tile {
    position: IVec3,
    scale: f32,
    spite_index: usize,
}

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct TileManager {
    tiles: HashMap<IVec2, (Tile, Entity)>,
    config: TileConfig,
    player_position: IVec2,
}

impl TileManager {
    fn new() -> Self {
        Self::empty()
    }

    fn empty() -> Self {
        return Self {
            tiles: HashMap::new(),
            config: TileConfig::default(),
            player_position: IVec2::splat(0),
        };
    }

    fn get_render_ranges(&self) -> (Range<i32>, Range<i32>) {
        return (
            (-(self.config.render_distance as i32 - self.player_position.x)
                ..self.config.render_distance as i32 + self.player_position.x),
            (-(self.config.render_distance as i32 - self.player_position.y)
                ..self.config.render_distance as i32 + self.player_position.y),
        );
    }
}

impl Default for TileManager {
    fn default() -> Self {
        return Self::empty();
    }
}

#[derive(Event, Default)]
pub struct PlayerTileChangeEvent;

fn spawn_chunk(mut ev_player_tile_change: EventWriter<PlayerTileChangeEvent>) {
    ev_player_tile_change.send_default();
}

fn detect_player_movement(
    player: Query<&Transform, With<Player>>,
    mut tile_manager: ResMut<TileManager>,
    mut ev_player_tile_change: EventWriter<PlayerTileChangeEvent>,
) {
    let old_tile_pos = tile_manager.player_position;

    let translation = player.single().translation;
    let tile_pos = pixel_to_tile_pos(&translation);


    if tile_pos != old_tile_pos {
        tile_manager.player_position = tile_pos;
        ev_player_tile_change.send_default();
    }
}

fn render_chunks_around_player(
    ev_player_tile_change: EventReader<PlayerTileChangeEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tile_manager: ResMut<TileManager>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if ev_player_tile_change.is_empty() {
        // info!("no tile change");
        return;
    }
    debug!("tile_change");

    let texture_handle = asset_server.load("tiles/sprite-sheet.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        TILE_SIZE,
        2,
        2,
        Some(Vec2 { x: 2.0, y: 2.0 }),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let (range_x, range_y) = tile_manager.get_render_ranges();

    let start = Instant::now();
    debug!("render_ranges: {:?} {:?}", range_x, range_y);
    let mut spite_index_mut: usize = 0;
    for x in range_x {
        for y in range_y.clone() {
            if tile_manager.tiles.contains_key(&IVec2::new(x, y)) {
                continue;
            }
            spite_index_mut += 1;
            if spite_index_mut >= 4 {
                spite_index_mut = 0;
            }

            let spite_index = spite_index_mut.clone();
            let tile = Tile {
                position: IVec3 { x, y, z: 0 },
                scale: 1.0,
                spite_index: spite_index,
            };

            let transform = Transform {
                translation: Vec3::new(
                    tile.position.x as f32 * TILE_SIZE.x + TILE_SIZE.x / 2.0,
                    tile.position.y as f32 * TILE_SIZE.y + TILE_SIZE.y / 2.0,
                    tile.position.z as f32,
                ),
                scale: Vec3::splat(tile.scale),
                ..default()
            };

            let entity = commands
                .spawn((
                    SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(spite_index),
                        texture_atlas: texture_atlas_handle.clone(),
                        transform: transform,
                        ..default()
                    },
                    tile,
                ))
                .id();

            tile_manager
                .tiles
                .insert(ivec3_into_ivec2(&tile.position), (tile, entity));
        }
    }
    let end_render = start.elapsed().as_millis();

    let start = Instant::now();
    let (range_x, range_y) = tile_manager.get_render_ranges();
    tile_manager.tiles.retain(|k, (tile, entity)| {
        let value_should_be_removed = !range_x.contains(&k.x) || !range_y.contains(&k.y);

        if value_should_be_removed {
            debug!("tile: {:?} found to remove", tile);
            commands.get_entity(*entity).unwrap().despawn();
        }

        return !value_should_be_removed;
    });

    let end = start.elapsed().as_millis();
    debug!(
        "tiles in manager: {} - retain took: {} nano-seconds render took: {} nano-seconds",
        tile_manager.tiles.len(),
        end,
        end_render
    );
}
