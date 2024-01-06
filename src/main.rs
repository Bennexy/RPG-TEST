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
    ops::Range,
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
    world::chunks::ChunkWorldGen,
};

use rand::{thread_rng, Rng};
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
        // .add_plugins(
        //     WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F2)),
        // )
        .add_plugins((
            ScaleableWorldViewPlugin,
            TreePlugin,
            PlayerPlugin,
            GameUI,
            ChunkWorldGen,
        ))
        .add_systems(Update, (close_on_esc,))
        // .add_systems(Startup, spawn_chunk)
        .run();
}
