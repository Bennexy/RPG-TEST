use bevy::{input::common_conditions::input_toggle_active, prelude::*, window::close_on_esc};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// local mods
mod pig;
mod ui;

// local uses
use pig::PigPlugin;
use ui::GameUI;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

enum ZoomState {
    MIN,
    MID,
    MAX,
}

#[derive(Component)]
pub struct WorldView {
    zoom_state: ZoomState,
}

impl WorldView {
    pub fn get_zoom_state(&self) -> bevy::render::camera::ScalingMode {
        match self.zoom_state {
            ZoomState::MIN => SCALING_MODE_MIN,
            ZoomState::MID => SCALING_MODE_MID,
            ZoomState::MAX => SCALING_MODE_MAX,
        }
    }

    pub fn increase_zoom_state(&mut self) {
        self.zoom_state = match self.zoom_state {
            ZoomState::MIN => ZoomState::MID,
            ZoomState::MID => ZoomState::MAX,
            ZoomState::MAX => ZoomState::MAX,
        }
    }
    pub fn decrease_zoom_state(&mut self) {
        self.zoom_state = match self.zoom_state {
            ZoomState::MIN => ZoomState::MIN,
            ZoomState::MID => ZoomState::MIN,
            ZoomState::MAX => ZoomState::MID,
        }
    }
}

#[derive(Resource)]
pub struct Money(pub f32);

impl Default for WorldView {
    fn default() -> Self {
        Self {
            zoom_state: ZoomState::MIN,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player { speed: 100.0 }
    }
}

impl Default for Money {
    fn default() -> Self {
        Self { 0: 100. }
    }
}

const SCALING_MODE_MIN: bevy::render::camera::ScalingMode =
    bevy::render::camera::ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };
const SCALING_MODE_MAX: bevy::render::camera::ScalingMode =
    bevy::render::camera::ScalingMode::AutoMin {
        min_width: 1024.0,
        min_height: 576.0,
    };
const SCALING_MODE_MID: bevy::render::camera::ScalingMode =
    bevy::render::camera::ScalingMode::AutoMin {
        min_width: 512.0,
        min_height: 288.0,
    };

fn main() {
    App::new()
        .init_resource::<Money>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Logic Farming Rougelike".into(),
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
        .add_plugins((PigPlugin, GameUI))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (character_movement, close_on_esc, change_world_scale),
        )
        .run();
}

fn setup(mut commands: Commands, asserts_server: Res<AssetServer>) {
    let mut camera: Camera2dBundle = Camera2dBundle::default();
    let world_view = WorldView::default();

    camera.projection.scaling_mode = world_view.get_zoom_state();

    commands.spawn((camera, world_view));

    let texture = asserts_server.load("character.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3 { z: 1., ..default() },
                ..Default::default()
            },
            texture,
            ..default()
        },
        Player::default(),
        Name::new("Player"),
    ));

    let texture = asserts_server.load("tree2.png");
    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                z: 0.5,
                x: 128.,
                y: 0.,
            },
            scale: Vec3 {
                x: 0.05,
                y: 0.05,
                z: 0.,
            },
            ..Default::default()
        },
        texture,
        ..default()
    },));

    let texture = asserts_server.load("tree.png");
    commands.spawn((SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                z: 0.5,
                x: -128.,
                y: 0.,
            },
            ..Default::default()
        },
        texture,
        ..default()
    },));
}

fn character_movement(
    mut player: Query<(&mut Transform, &Player), (Without<WorldView>, With<Player>)>,
    mut world_view: Query<&mut Transform, (Without<Player>, With<WorldView>)>,
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

fn change_world_scale(
    mut world_view: Query<
        (&mut OrthographicProjection, &mut WorldView),
        (Without<Player>, With<WorldView>),
    >,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlLeft) {
        // info!("in control loop");
        let (mut projection, mut world_view) = world_view.single_mut();
        // buttons.pressed(MouseButton::Other(2))
        if input.just_pressed(KeyCode::Minus) {
            world_view.increase_zoom_state();
            projection.scaling_mode = world_view.get_zoom_state();
        };
        if input.just_pressed(KeyCode::Plus) {
            world_view.decrease_zoom_state();
            projection.scaling_mode = world_view.get_zoom_state();
        };
    }
}
