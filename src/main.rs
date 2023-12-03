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
    player::{setup as spawn_player, Player},
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
        .add_plugins((ScaleableWorldViewPlugin, TreePlugin))
        .add_systems(Startup, (setup))
        .add_systems(
            Update,
            (
                close_on_esc,
                // change_world_scale,
                change_game_state,
                character_movement,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let mut camera: Camera2dBundle = Camera2dBundle::default();
    // let world_view = WorldView::default();

    // camera.projection.scaling_mode = world_view.get_zoom_state();

    // commands.spawn((camera, world_view));

    // let background_image= asset_server.load("background.png");
    // commands
    //     .spawn(SpriteBundle {
    //         texture: materials.add(background_image),
    //         ..Default::default()
    //     });

    let texture = asset_server.load("images/player-v1.png"); //character.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 {
                    z: 1.0,
                    ..default()
                },
                ..Default::default()
            },
            texture,
            ..default()
        },
        Player::default(),
        Name::new("Player"),
    ));
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

pub fn character_movement(
    mut player: Query<(&mut Transform, &Player), With<Player>>,
    mut world_view: Query<&mut Transform, (With<WorldView>, Without<Player>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut tansform_player, player) = player.single_mut();
    let mut tansform_world_view = world_view.single_mut();

    // for (mut transform, player) in &mut player.into {
    let movement_amount = player.speed * time.delta_seconds();

    let mut move_x: f32 = 0.;
    let mut move_y: f32 = 0.;

    if input.pressed(KeyCode::W) {
        move_y += movement_amount;
    }
    if input.pressed(KeyCode::S) {
        move_y -= movement_amount;
    }
    if input.pressed(KeyCode::D) {
        move_x += movement_amount;
    }
    if input.pressed(KeyCode::A) {
        move_x -= movement_amount;
    }

    if move_x != 0. && move_y != 0. {
        move_x /= 2.;
        move_y /= 2.;
    }

    if input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight) {
        move_x *= 2.;
        move_y *= 2.;
    }

    tansform_player.translation.y += move_y;
    tansform_player.translation.x += move_x;
    tansform_world_view.translation.y += move_y;
    tansform_world_view.translation.x += move_x;
}
