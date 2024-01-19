use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter, Write},
    ops::Range,
    path::Path,
    time::Instant,
};

use bevy::prelude::*;
use bincode;
use noise::{NoiseFn, Perlin};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::{
    consts::{CHUNK_SIZE, NOISE_SCALE, RENDER_DISTANCE, SPITE_SHEET_COLUMNS, TILE_SIZE},
    game_plugins::{
        player::Player,
        world::{
            helpers::{
                ivec3_into_ivec2, load_texture_atlas_handel, pixel_to_chunk_pos, pixel_to_tile_pos,
                tile_to_chunk_pos,
            },
            type_tiles::TileType, world_gen::get_tile_type_for_pos,
        },
    },
};

impl Default for ChunkConfig {
    fn default() -> Self {
        Self {
            seeds: Seeds::default(),
            render_distance: 8,
            chunk_size: 16,
        }
    }
}

pub struct ChunkWorldGen;

impl Plugin for ChunkWorldGen {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerChunkChangeEvent>()
            .init_resource::<ChunkManager>()
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                (
                    detect_player_chunk_change.before(draw_chunks_around_player),
                    draw_chunks_around_player,
                ),
            );
    }
}

#[derive(Event, Default)]
pub struct PlayerChunkChangeEvent;

#[derive(Component, Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Tile {
    position: IVec3,
    scale: f32,
    spite_index: usize,
}

impl Tile {
    // Gets the pixel position on a global map x,y,z
    // This is the position to render the tile at.
    fn get_translation(
        tile_position_in_chunk: &IVec2,
        config: &ChunkConfig,
        chunk_pos: &IVec2,
    ) -> Vec3 {
        let translation = Vec3 {
            x: (tile_position_in_chunk.x as f32 * TILE_SIZE.x + TILE_SIZE.x / 2.0)
                + (chunk_pos.x as f32 * config.chunk_size as f32 * TILE_SIZE.x),
            y: (tile_position_in_chunk.y as f32 * TILE_SIZE.y + TILE_SIZE.y / 2.0)
                + (chunk_pos.y as f32 * config.chunk_size as f32 * TILE_SIZE.x),
            z: 0.0,
        };

        return translation;
    }

    // Gets the tile position on a gloabal map x,y
    fn get_global_tile_position(
        tile_position_in_chunk: &IVec2,
        config: &ChunkConfig,
        chunk_pos: &IVec2,
    ) -> IVec2 {
        let trans = Tile::get_translation(tile_position_in_chunk, config, chunk_pos);
        return pixel_to_tile_pos(&trans);
    }
}


#[derive(Component, Serialize, Deserialize, Debug)]
pub struct Chunk {
    position: IVec2,
    tiles: HashMap<IVec2, (Tile, Entity)>,
}

impl Chunk {
    fn generate_new_chunk(chunk_pos: &IVec2, config: &ChunkConfig) -> HashMap<IVec2, Tile> {
        let mut tiles: HashMap<IVec2, Tile> =
            HashMap::with_capacity(config.chunk_size.pow(2) as usize);

        let mut rng = thread_rng();
        // let perlin = Perlin::new(config.seed);

        for x in 0..config.chunk_size as i32 {
            for y in 0..config.chunk_size as i32 {
                let pos: IVec2 = IVec2::new(x, y);
                let perlin_pos_vec2 = Tile::get_global_tile_position(&pos, config, chunk_pos);

                // let perlin_chunk_pos = [
                //     (chunk_pos.x as f64 + 0.1) / NOISE_SCALE,
                //     (chunk_pos.y as f64 + 0.1) / NOISE_SCALE,
                // ];
                // let perlin_chunk_offset = perlin.get(perlin_chunk_pos);
                // // info!("{:?}", perlin_pos);
                // let perlin_pos = [
                //     ((perlin_pos_vec2.x as f64) / NOISE_SCALE) + perlin_chunk_offset,
                //     ((perlin_pos_vec2.y as f64) / NOISE_SCALE) + perlin_chunk_offset,
                // ];

                // let perlin_value =
                //     perlin.get(perlin_pos) / perlin_chunk_offset + perlin_chunk_offset * 2.0;

                let si = get_tile_type_for_pos(&perlin_pos_vec2, &config);
                // debug!(
                //     "{:?}, {:?}, {:?}, {:?}",
                //     si, perlin_pos, perlin_value, perlin_pos_vec2
                // );
                
                
                let si =
                    (si.to_usize() * SPITE_SHEET_COLUMNS) + rng.gen_range(0..SPITE_SHEET_COLUMNS);

                let tile = Tile {
                    position: IVec3 { x: x, y: y, z: 0 },
                    scale: 1.0,
                    spite_index: si,
                };

                tiles.insert(pos, tile);
                // info!(si);
            }
        }

        return tiles;
    }

    fn despawn_chunk(&self, commands: &mut Commands) {
        self.tiles.iter().for_each(|(_, (_, entity))| {
            commands.get_entity(*entity).unwrap().despawn();
        });
        // todo!("implement save function");
    }

    fn render(
        chunk_pos: IVec2,
        raw_data: HashMap<IVec2, Tile>,
        config: &ChunkConfig,
        commands: &mut Commands,
        texture_atlas_handle: Handle<TextureAtlas>,
    ) -> Self {
        let mut selfer = Self {
            position: chunk_pos,
            tiles: HashMap::with_capacity(raw_data.len()),
        };

        for (pos, tile) in raw_data.iter() {
            let translation = Tile::get_translation(pos, config, &selfer.position);

            let transform = Transform {
                translation: translation,
                scale: Vec3::splat(tile.scale),
                ..default()
            };
            let entity = commands
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(tile.spite_index),
                        transform: transform,
                        ..default()
                    },
                    *tile,
                ))
                .id();

            selfer.tiles.insert(*pos, (*tile, entity));
        }

        return selfer;
    }

    fn get_file_string(chunk_pos: &IVec2, config: &ChunkConfig) -> String {
        let file_string = format!(
            "save/chunk_{}_{}_{}.bin",
            config.chunk_size, chunk_pos.x, chunk_pos.y
        );

        return file_string;
    }

    fn load(chunk_pos: IVec2, config: &ChunkConfig) -> Option<HashMap<IVec2, Tile>> {
        let fp = Self::get_file_string(&chunk_pos, config);
        let file_path = Path::new(&fp);
        if !file_path.exists() {
            return None;
        }

        let reader = BufReader::new(File::open(file_path).unwrap());

        let data: HashMap<IVec2, Tile> = bincode::deserialize_from(reader).unwrap();
        return Some(data);
    }

    fn save(&self, config: &ChunkConfig) {
        let fp = Self::get_file_string(&self.position, config);
        let file_path = Path::new(&fp);
        let file = File::create(&file_path).unwrap_or(File::open(&file_path).unwrap());

        let mut data_to_save = HashMap::with_capacity(self.tiles.len());

        let mut writer = BufWriter::new(file);
        for (key, (tile, _)) in self.tiles.iter() {
            data_to_save.insert(key, tile);
        }

        bincode::serialize_into(&mut writer, &data_to_save).unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Seeds {
    pub biom: u32,
    pub tiles: u32,
}

impl Default for Seeds {
    fn default() -> Self {
        return Self {
            biom: 3654,
            tiles: 97123,
        };
    }
}

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ChunkConfig {
    pub render_distance: u32,
    pub chunk_size: u32,
    pub seeds: Seeds,
}

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct ChunkManager {
    chunks: HashMap<IVec2, Chunk>,
    config: ChunkConfig,
    player_position: IVec2,
}

impl ChunkManager {
    fn empty() -> Self {
        Self {
            chunks: HashMap::new(),
            config: ChunkConfig::default(),
            player_position: IVec2::splat(0),
        }
    }

    fn get_render_ranges(&self) -> (Range<i32>, Range<i32>) {
        return (
            (-(self.config.render_distance as i32 - self.player_position.x)
                ..self.config.render_distance as i32 + self.player_position.x),
            (-(self.config.render_distance as i32 - self.player_position.y)
                ..self.config.render_distance as i32 + self.player_position.y),
        );
    }

    fn despawn_chunk(&mut self, commands: &mut Commands, chunk_pos: &IVec2) {
        let (_, chunk) = self.chunks.remove_entry(chunk_pos).unwrap();
        chunk.save(&self.config);
        chunk.despawn_chunk(commands);
    }
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self::empty()
    }
}

fn startup(mut ev_player_chunk_change: EventWriter<PlayerChunkChangeEvent>) {
    ev_player_chunk_change.send_default();
}

fn detect_player_chunk_change(
    mut ev_player_chunk_change: EventWriter<PlayerChunkChangeEvent>,
    player: Query<&Transform, With<Player>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let player_position = player.single().translation;

    let tile_pos = pixel_to_tile_pos(&player_position);
    let chunk_pos = pixel_to_chunk_pos(&player_position, &chunk_manager.config);
    debug!("tile_pos: {:?}, chunk_pos: {:?}", tile_pos, chunk_pos);
    if chunk_manager.player_position != chunk_pos {
        ev_player_chunk_change.send_default();
        debug!("changed player chunk to {chunk_pos}");
        chunk_manager.player_position = chunk_pos;
    }
}

fn draw_chunks_around_player(
    asset_server: Res<AssetServer>,
    mut ev_player_chunk_change: EventReader<PlayerChunkChangeEvent>,
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if ev_player_chunk_change.is_empty() {
        return;
    }
    ev_player_chunk_change.clear(); // only run once

    debug!("tile_change");

    let texture_atlas_handle: Handle<TextureAtlas> =
        load_texture_atlas_handel(&asset_server, &mut texture_atlases);

    let (range_x, range_y) = chunk_manager.get_render_ranges();

    let start = Instant::now();
    debug!("render_ranges: {:?} {:?}", range_x, range_y);

    for x in range_x {
        for y in range_y.clone() {
            let chunk_pos = IVec2::new(x, y);
            if chunk_manager.chunks.contains_key(&chunk_pos) {
                continue;
            }

            let chunk_data = match Chunk::load(chunk_pos, &chunk_manager.config) {
                Some(chunk) => {
                    debug!("loaded chunk {}", chunk_pos);
                    chunk
                }
                None => {
                    let chunk = Chunk::generate_new_chunk(&chunk_pos, &chunk_manager.config);
                    debug!("generated chunk {}", chunk_pos);
                    chunk
                }
            };

            let chunk = Chunk::render(
                chunk_pos,
                chunk_data,
                &chunk_manager.config,
                &mut commands,
                texture_atlas_handle.clone(),
            );

            chunk_manager.chunks.insert(chunk_pos, chunk);
        }
    }
    let end_render = start.elapsed().as_millis();

    // start retain
    let start = Instant::now();
    let (range_x, range_y) = chunk_manager.get_render_ranges();

    let chunk_config = chunk_manager.config.clone();

    chunk_manager.chunks.retain(|k, chunk| {
        let value_should_be_removed = !range_x.contains(&k.x) || !range_y.contains(&k.y);

        if value_should_be_removed {
            // chunk.save(&chunk_config);
            chunk.despawn_chunk(&mut commands);
            debug!("chunk: {:?} found to remove", k);
        }

        return !value_should_be_removed;
    });

    let end = start.elapsed().as_millis();
    info!(
        "chunks in manager: {} - tiles in manager: {} - retain took: {} mili-seconds render took: {} mili-seconds",
        chunk_manager.chunks.len(),
        chunk_manager.chunks.len() as usize * chunk_manager.config.chunk_size.pow(2) as usize,
        end,
        end_render
    );
}
