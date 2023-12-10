use bevy::{
    input::common_conditions::input_toggle_active,
    prelude::*,
    window::close_on_esc,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod game_plugins;
// mod pig;
mod ui;
mod zoom;
// local uses
use game_plugins::{
    player::{Player, PlayerPlugin},
    tree::TreePlugin,
    world_map::world_gen::WorldGenPlugin,
};

use ui::GameUI;
use zoom::ScaleableWorldViewPlugin;

mod consts;

fn main() {
    App::new()
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
            WorldGenPlugin,
        ))
        .add_systems(
            Update,
            (
                close_on_esc,
                // spawn_chunks_around_camera2,
                // spawn_chunks_around_camera
                // change_world_scale,
                // character_movement
                //     despawn_outofrange_chunks,
            ),
        )
        // .add_systems(Startup, tile_map_6init)
        .run();
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    player_pos: Query<(&mut Transform, Entity), With<Player>>,
) {
    let _ = player_pos.single();
    info!("got chunks from main");
}
