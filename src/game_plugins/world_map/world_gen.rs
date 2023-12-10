use bevy::{
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_ecs_tilemap::{map::TilemapRenderSettings, TilemapPlugin};

use rand::{thread_rng, Rng};

use crate::{consts::RENDER_CHUNK_SIZE, game_plugins::player::Player, zoom::WorldView};

use super::{chunk_gen::spawn_chunks, utils::world_to_chunks};

#[derive(Resource)]
pub struct RngJesus {
    pub seed: usize,
}

impl Default for RngJesus {
    fn default() -> Self {
        Self {
            seed: thread_rng().gen(),
        }
    }
}

impl RngJesus {
    fn new_fixed() -> Self {
        Self { seed: 1234567890 }
    }
}

#[derive(Component)]
pub struct Tile;

#[derive(Resource, Default, Debug, Clone)]
pub struct ChunkManager {
    pub spawned_tiles: HashSet<IVec2>,
    pub spawned_chunks: HashMap<IVec2, Entity>,
}

impl ChunkManager {
    pub fn add_new_chunk(&mut self, vec: IVec2, entity: Entity) {
        self.spawned_tiles.insert(vec);
        self.spawned_chunks.insert(vec, entity);
    }

    pub fn remove_chunk(&mut self, vec: &IVec2) -> Option<Entity> {
        self.spawned_tiles.remove(vec);
        self.spawned_chunks.remove(vec)
    }

    pub fn contains(&self, vec: &IVec2) -> bool {
        self.spawned_tiles.contains(vec)
    }
}

#[derive(Component)]
pub struct TileMap;

pub struct WorldGenPlugin;

impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChunkManager>()
            .init_resource::<RngJesus>()
            // .insert_resource(ChunkManager::default())
            // .insert_resource(RngJesus::default())
            // `TilemapRenderSettings` must be added before the `TilemapPlugin`.
            .insert_resource(TilemapRenderSettings {
                render_chunk_size: RENDER_CHUNK_SIZE,
                ..Default::default()
            })
            .add_plugins(TilemapPlugin)
            // .add_systems(Update, (spawn_chunks_around_camera2,));
            .add_systems(
                Update,
                (
                    spawn_chunks_around_camera,
                    despawn_chunks_out_of_range_of_camera,
                ),
            );
    }
}


pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut chunk_manager: ResMut<ChunkManager>,
    seed: Res<RngJesus>,
    player_pos: Query<&Transform, With<Player>>,
) {

    let player_pos = player_pos.single();
    let (chunk_x, chunk_y) = world_to_chunks((player_pos.translation.x, player_pos.translation.y));

    for x in chunk_x - RENDER_CHUNK_SIZE.x as i32..chunk_x + RENDER_CHUNK_SIZE.x as i32 {
        for y in chunk_y - RENDER_CHUNK_SIZE.y as i32..chunk_y + RENDER_CHUNK_SIZE.y as i32 {
            let chunk = IVec2::new(x, y);

            if !chunk_manager.contains(&chunk) {
                let entity = spawn_chunks(&mut commands, &asset_server, &seed, chunk);
                chunk_manager.add_new_chunk(chunk, entity);
            }
        }
    }
}

fn despawn_chunks_out_of_range_of_camera(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    player_pos: Query<&mut Transform, With<Player>>,
) {
    let player_pos = player_pos.single();
    let (chunk_x, chunk_y) = world_to_chunks((player_pos.translation.x, player_pos.translation.y));

    let mut allowed_ivec2s: Vec<IVec2> = Vec::new();

    for x in chunk_x - RENDER_CHUNK_SIZE.x as i32..chunk_x + RENDER_CHUNK_SIZE.x as i32 {
        for y in chunk_y - RENDER_CHUNK_SIZE.y as i32..chunk_y + RENDER_CHUNK_SIZE.y as i32 {
            let chunk = IVec2::new(x, y);
            allowed_ivec2s.push(chunk);
        }
    }

    for val in chunk_manager.spawned_tiles.clone() {
        if !allowed_ivec2s.contains(&val) {
            let entity = chunk_manager.remove_chunk(&val);
            match entity {
                Some(val) => commands.entity(val).despawn_recursive(),
                None => error!("Tried to delete chunk {:?}- failed", val),
            }
        }
    }
}
