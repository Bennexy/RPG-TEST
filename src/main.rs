use bevy::{
    input::{common_conditions::input_toggle_active, mouse::MouseWheel},
    prelude::*,
    window::close_on_esc,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// local mods

mod game_plugins;
mod pig;
mod ui;
mod zoom;

// use game_plugins::player::PlayerPlugin;
use zoom::{change_world_scale, WorldView, ScaleableWorldViewPlugin};

// local uses
use game_plugins::{
    player::{setup as spawn_player, Player, PlayerPlugin},
    tree::TreePlugin,
};
use pig::PigPlugin;
use ui::GameUI;

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Resource, States, Debug, Hash, PartialEq, Eq, Clone, Reflect)]
pub enum GameState {
    MENU,
    GAME,
}

impl Default for GameState {
    fn default() -> Self {
        Self::GAME
    }
}

impl Default for Money {
    fn default() -> Self {
        Self { 0: 100. }
    }
}

fn main() {
    App::new()
        .init_resource::<Money>()
        .add_state::<GameState>()
        .init_resource::<State<GameState>>()
        .init_resource::<NextState<GameState>>()
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
        .add_plugins((ScaleableWorldViewPlugin, TreePlugin, PlayerPlugin))
        // .add_systems(Startup, (setup))
        .add_systems(
            Update,
            (
                close_on_esc,
                // change_world_scale,
                change_game_state,
                // character_movement,
            ),
        )
        .run();
}


fn change_game_state(
    game_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<Input<KeyCode>>,
) {
    if !input.just_pressed(KeyCode::M) {
        return;
    }

    // let mut game_state: &GameState = game_state.get();

    let new = match game_state.get() {
        GameState::GAME => GameState::MENU,
        GameState::MENU => GameState::GAME,
    };
    next_state.set(new);
    // if game_state.get() == &new {return};

    // let boxed = Box::new(new);
    // let boxed2 = Box::new(boxed.as_reflect());

    // match game_state.set(boxed2) {
    //     Ok(_) => info!("successfully set new game_state"),
    //     Err(e) => error!("cant set new game state {:?}", e)
    // }

    info!("game state {:?}", game_state);
}

