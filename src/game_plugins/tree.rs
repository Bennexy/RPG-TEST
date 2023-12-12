use super::player::Player;

use bevy::{prelude::*, audio::VolumeLevel};
use bevy_turborand::prelude::*;

#[derive(Component, Reflect)]
pub struct TreeParent;

#[derive(Component, Reflect)]
pub struct Tree {
    health: usize,
}

impl Default for Tree {
    fn default() -> Self {
        return Self { health: 10 };
    }
}

#[derive(Resource, Default)]
pub struct TreeCount(pub usize);

pub struct TreePlugin;

impl Plugin for TreePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeCount>()
            .init_resource::<GlobalRng>()
            // .add_systems(Startup, spawn_trees)
            .add_systems(Update, tree_hit)
            .register_type::<Tree>();
    }
}

fn spawn_trees(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
) {
    commands
        .spawn((
            SpatialBundle::default(),
            TreeParent,
            RngComponent::from(&mut global_rng),
            Name::new("Tree Parent"),
        ))
        .with_children(|commands| {
            for _ in 0..20 {
                let x = global_rng.isize(-600..600) as f32;
                let y = global_rng.isize(-600..600) as f32;
                let texture = asset_server.load("images/tree2.png");
                commands.spawn((
                    SpriteBundle {
                        texture,
                        transform: Transform {
                            translation: Vec3 { x, y, z: 0.0 },
                            scale: Vec3 {
                                x: 0.05,
                                y: 0.05,
                                z: 0.,
                            },
                            ..default()
                        },
                        ..default()
                    },
                    Tree::default(),
                    Name::new("Tree"),
                ));
            }
        });
}

fn tree_hit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player: Query<&Transform, With<Player>>,
    mut tree_parent: Query<(Entity, &mut RngComponent), With<TreeParent>>,
    mut trees: Query<(Entity, &Transform, &mut Tree), With<Tree>>,
    input: Res<Input<KeyCode>>,
) {

    if !input.just_pressed(KeyCode::Space) {
        return
    }
    let (tree_parent, mut rng) = tree_parent.single_mut();

    let player = player.single();

    let mut player_cords_max = player.translation.clone();
    let mut player_cords_min = player.translation.clone();

    player_cords_max.x = player_cords_max.x + 20.;
    player_cords_max.y = player_cords_max.y + 20.;

    player_cords_min.x = player_cords_min.x - 20.;
    player_cords_min.y = player_cords_min.y - 20.;

    let mut el_count: usize = 0;
    let mut new_trees: usize = 0;

    for (entity, transform, mut tree) in &mut trees {
        let diff = player.translation - transform.translation;
        el_count += 1;

        
        if     diff.x <= 20.
            && diff.x >= -20.
            && diff.y <= 20.
            && diff.y >= -20.
        {
            // println!("tree!tree!tree!tree!tree!tree!tree!tree!tree!tree!tree!");
            let v = tree.health as isize;
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/Dump.ogg"),
                settings: PlaybackSettings {
                    volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.05)),
                    ..default()
                }
            });
            if v -5 <= 0 {
                commands.entity(tree_parent).remove_children(&[entity]).with_children(|commands| {
                    for _ in 0..2 {
                        let x = rng.isize(-2000..2000) as f32;
                        let y = rng.isize(-2000..2000) as f32;
                        let texture = asset_server.load("images/tree-pine-v1.png");
                        commands.spawn((
                            SpriteBundle {
                                texture,
                                transform: Transform {
                                    translation: Vec3 { x, y, z: 0.0 },
                                    scale: Vec3 {
                                        x: 1.,
                                        y: 1.,
                                        z: 0.,
                                    },
                                    ..default()
                                },
                                ..default()
                            },
                            Tree::default(),
                            Name::new("Tree"),
                        ));
                        new_trees += 1;
                    }
                });
                commands.entity(entity).despawn();
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/yeah_budy.ogg"),
                    settings: PlaybackSettings {
                        volume: bevy::audio::Volume::Relative(VolumeLevel::new(0.05)),
                        ..default()
                    }
                });
            } else {
                tree.health -= 5;
            }
        } //else {println!("no tree here")}
    }

    if new_trees != 0 {
        info!("spawen {} new trees. Total tree cound:{}", new_trees, el_count);
    };

}
