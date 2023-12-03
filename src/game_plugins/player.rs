use bevy::prelude::*;

use crate::zoom::WorldView;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, character_movement);
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player { speed: 100.0 }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn character_movement(
    mut player: Query<(&mut Transform, &Player), (With<Player>, Without<WorldView>)>,
    mut world_view: Query<&mut Transform, (With<WorldView>, Without<Player>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut tansform_player, player) = player.single_mut();
    let mut tansform_world_view = world_view.single_mut();

    // for wv in &mut world_view {
    //     info!("{:?}", wv);
    // }
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
