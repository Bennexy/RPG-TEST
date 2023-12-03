use bevy::{input::{mouse::MouseWheel, common_conditions::input_toggle_active}, prelude::*, window::close_on_esc};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// local mods

mod pig;
mod game_plugins;
mod ui;
mod zoom;

use zoom::{WorldView, change_world_scale};
use game_plugins::player::PlayerPlugin;

// local uses
use pig::PigPlugin;
use game_plugins::{tree::TreePlugin, player::{Player, setup as spawn_player}};
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
        .add_plugins((PlayerPlugin, TreePlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                close_on_esc,
                change_world_scale,
                change_game_state,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera: Camera2dBundle = Camera2dBundle::default();
    let world_view = WorldView::default();

    camera.projection.scaling_mode = world_view.get_zoom_state();

    commands.spawn((camera, world_view));

    // let background_image= asset_server.load("background.png");
    // commands
    //     .spawn(SpriteBundle {
    //         texture: materials.add(background_image),
    //         ..Default::default()
    //     });

    // let texture = asset_server.load("images/player-v1.png"); //character.png");
    // commands.spawn((
    //     SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3 {
    //                 z: 1.0,
    //                 ..default()
    //             },
    //             ..Default::default()
    //         },
    //         texture,
    //         ..default()
    //     },
    //     Player::default(),
    //     Name::new("Player"),
    // ));
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
