use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    utils::{HashMap, HashSet},
    window::close_on_esc,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use noise::{NoiseFn, Perlin};

use rand::{prelude::SliceRandom, thread_rng, Rng};

mod game_plugins;
// mod pig;
mod ui;
mod zoom;
// local uses
use game_plugins::{
    player::{self, Player, PlayerPlugin},
    tree::TreePlugin,
};
use rpg_game::map::utils::{chunks_to_world, world_to_chunks, world_to_tiles};
use ui::GameUI;
use zoom::ScaleableWorldViewPlugin;

mod consts;
use consts::*;

#[derive(Resource)]
struct RngJesus {
    seed: usize,
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
// use pig::PigPlugin;

#[derive(Component)]
pub struct Tile;

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_tiles: HashSet<IVec2>,
    // pub spawned_chunks: HashMap<IVec2, Entity>,
}

// impl ChunkManager {
//     fn add_new_chunk(&mut self, vec: IVec2, entity: Entity) {
//         self.spawned_tiles.insert(vec);
//         self.spawned_chunks.insert(vec, entity);
//     }

//     fn remove_chunk(&mut self, vec: &IVec2) -> Option<Entity> {
//         self.spawned_tiles.remove(vec);
//         self.spawned_chunks.remove(vec)
//     }

//     fn contains(&self, vec: &IVec2) -> bool {
//         self.spawned_tiles.contains(vec)
//     }
// }

#[derive(Component)]
struct TileMap;

fn main() {
    App::new()
        .init_resource::<RngJesus>()
        // .init_resource::<SeededRng>()
        // `TilemapRenderSettings` be added before the `TilemapPlugin`.
        .insert_resource(TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        })
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
        .add_plugins(TilemapPlugin)
        .insert_resource(ChunkManager::default())
        .add_plugins((ScaleableWorldViewPlugin, TreePlugin, PlayerPlugin, GameUI))
        .add_systems(
            Update,
            (
                close_on_esc,
                // change_world_scale,
                // character_movement,
                spawn_chunks_around_camera,
                // despawn_chunks_out_of_range_of_camera,
                //     despawn_outofrange_chunks,
            ),
        )
        // .add_systems(Startup, tile_map_6init)
        .run();
}

fn spawn_chunks(
    commands: &mut Commands,
    asset_server: &AssetServer,
    seed: &RngJesus,
    chunk_position: IVec2,
) {
    // chunk_manager.spawned_chunks.insert(IVec2::new(x, y));

    let tilemap_entity = commands.spawn_empty().insert(TileMap).id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    let image_handles = vec![
        asset_server.load("tiles/grass/grass_32x32_0.png"),
        asset_server.load("tiles/water.png"),
    ];
    let texture_vec = TilemapTexture::Vector(image_handles);

    let perlin = Perlin::new(seed.seed as u32);

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_pos = TilePos { x, y };


            let (world_x, world_y) = chunks_to_world(chunk_position, tile_pos);
            info!("x: {}-{}, y: {}-{}", x, world_x, y,world_y);

            let perlin_value =
                perlin.get([world_x as f64 / NOISE_SCALE, world_y as f64 / NOISE_SCALE]);
            let texture_index: usize = if perlin_value > 0.3 { 1 } else { 1 };

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

    let transform = Transform::from_translation(Vec3::new(
        chunk_position.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_position.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.0,
    ));
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: texture_vec,
        tile_size: TILE_PIXEL_SIZE,
        transform: transform,
        ..Default::default()
    });
}

fn spawn_chunks_around_camera(
    mut commands: Commands,
    mut chunk_manager: ResMut<ChunkManager>,
    asset_server: Res<AssetServer>,
    seed: Res<RngJesus>,
    player_pos: Query<&mut Transform, With<Player>>,
) {
    let player_pos = player_pos.single();
    let (chunk_x, chunk_y) = world_to_chunks((player_pos.translation.x, player_pos.translation.y));

    // info!("x:{}-chunk_x:{}, y:{}-chunk_y:{}", plsayer_pos.translation.x, chunk_x, player_pos.translation.y, chunk_y);
    // let chunk = IVec2::new(chunk_x, chunk_y);

    // spawn_chunks(&mut commands, &asset_server, &seed, chunk);
    // chunk_manager.spawned_tiles.insert(chunk); //, tile_map_entity);

    for x in chunk_x - RENDER_CHUNK_SIZE.x as i32..chunk_x + RENDER_CHUNK_SIZE.x as i32 {
        for y in chunk_y - RENDER_CHUNK_SIZE.y as i32..chunk_y + RENDER_CHUNK_SIZE.y as i32 {
            let chunk = IVec2::new(x, y);

            // info!("new chunk check: {} {:?}", chunk, chunk_manager.spawned_tiles);
            if !chunk_manager.spawned_tiles.contains(&chunk) {
                // info!("new chunk: {} {}", chunk.x, chunk.y);
                // chunk_manager.spawned_chunks.insert(chunk);

                spawn_chunks(&mut commands, &asset_server, &seed, chunk);
                chunk_manager.spawned_tiles.insert(chunk); //, tile_map_entity);
            }
        }
    }
}

// fn despawn_chunks_out_of_range_of_camera(
//     mut commands: Commands,
//     mut chunk_manager: ResMut<ChunkManager>,
//     player_pos: Query<&mut Transform, With<Player>>,
// ) {
//     let player_pos = player_pos.single().translation;
//     let (chunk_x, chunk_y) = world_to_chunks((player_pos.x, player_pos.y));
//     let player_pos = IVec2::new(player_pos.x.floor() as i32, player_pos.y.floor() as i32);

//     let mut allowed_ivec2s: Vec<IVec2> = Vec::new();
//     for x in player_pos.x - RENDER_CHUNK_SIZE.x as i32..player_pos.x + RENDER_CHUNK_SIZE.x as i32 {
//         for y in
//             player_pos.y - RENDER_CHUNK_SIZE.y as i32..player_pos.y + RENDER_CHUNK_SIZE.y as i32
//         {
//             allowed_ivec2s.push(IVec2 { x, y });
//         }
//     }
//     let values_to_remove: Vec<_> = chunk_manager
//         .spawned_tiles
//         .iter()
//         .filter(|val| allowed_ivec2s.contains(val))
//         .map(|val| {
//             IVec2::new(val.x, val.y)
//         })
//         .collect();

//     for value in values_to_remove.iter() {
//         // let entity = chunk_manager.remove_chunk(&value).unwrap();
//         // commands.entity(entity).despawn_recursive();
//     }
// }

fn _tile_map_init(mut commands: Commands, asset_server: Res<AssetServer>, seed: Res<RngJesus>) {
    let perlin = Perlin::new(seed.seed as u32);
    // rng.

    let map_size = TilemapSize { x: 512, y: 512 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().insert(TileMap).id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    let image_handles = vec![
        asset_server.load("tiles/grass/grass_32x32_0.png"),
        asset_server.load("tiles/water.png"),
    ];
    let texture_vec = TilemapTexture::Vector(image_handles);

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let res = perlin.get([x as f64 / NOISE_SCALE, y as f64 / NOISE_SCALE]);
            let texture_index = if res > 0.5 { 1 } else { 0 };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex {
                        0: texture_index as u32,
                    },
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: texture_vec,
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, -1.0),
        ..Default::default()
    });
}
