use bevy::prelude::*;

use crate::{zoom::WorldView, consts::TILE_SIZE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, character_movement);
    }
}


fn pixel_to_tile_pos(position: &Vec3) -> IVec2 {
    let x_res = (position.x) / TILE_SIZE.x ;
    let y_res = (position.y) / TILE_SIZE.y;

    let x = if position.x.is_sign_positive() {
        (x_res).ceil() as i32
    } else {
        (x_res).floor() as i32
    };
    let y = if position.y.is_sign_positive() {
        (y_res).ceil() as i32
    } else {
        (y_res).floor() as i32
    };


    return IVec2 { x, y };
    // let tile_pos = IVec2::new(
    //     x: position.x / TILE_SIZE,
    //     y: position.y / TILE_SIZE,
    // )
}


#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub tile_position: IVec2
}

impl Default for Player {
    fn default() -> Self {
        Player { speed: 100.0, tile_position: IVec2::splat(0) }
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
    mut player: Query<(&mut Transform, &mut Player), (With<Player>, Without<WorldView>)>,
    mut world_view: Query<&mut Transform, (With<WorldView>, Without<Player>)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut tansform_player, mut player) = player.single_mut();
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
    if input.pressed(KeyCode::ControlLeft) || input.pressed(KeyCode::ControlRight) {
        move_x *= 16.;
        move_y *= 16.;
    }
    if input.pressed(KeyCode::ControlLeft) && input.pressed(KeyCode::ControlRight) {
        move_x *= 4.;
        move_y *= 4.;
    }

    tansform_player.translation.y += move_y;
    tansform_player.translation.x += move_x;
    tansform_world_view.translation.y += move_y;
    tansform_world_view.translation.x += move_x;


    player.tile_position = pixel_to_tile_pos(&tansform_player.translation);
}
