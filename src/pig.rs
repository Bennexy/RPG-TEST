use crate::{Money, Player};
use bevy::{audio::PlaybackMode, prelude::*};

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PigCount>()
            .add_systems(Startup, spawn_pig_parent)
            .add_systems(Update, (spawn_pig, pig_lifetime))
            .register_type::<Pig>();
    }
}

#[derive(Resource)]
pub struct PigCount(pub usize);

impl Default for PigCount {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

#[derive(Component)]
pub struct PigParent;

fn spawn_pig_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), PigParent, Name::new("Pig Parent")));
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    mut pig_count: ResMut<PigCount>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<PigParent>>,
) {
    if !input.pressed(KeyCode::ShiftRight) && !input.just_pressed(KeyCode::Space) {
        return;
    }
    if money.0 <= 10.0 {
        return;
    }

    pig_count.0 += 1;

    let player_transform = player.single();

    let parent = parent.single();

    money.0 -= 10.0;
    debug!("Spent $10 on a pig, remaining money: ${:?}", money.0);

    let texture = asset_server.load("pig.png");

    commands.entity(parent).with_children(|commands| {
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3 {
                        x: player_transform.translation.x,
                        y: player_transform.translation.y,
                        z: 0.0,
                    },
                    rotation: player_transform.rotation,
                    scale: Vec3 {
                        x: 0.75,
                        y: 0.75,
                        z: 0.,
                    },
                },
                ..default()
            },
            Pig {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
            Name::new("Pig"),
        ));

        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/zipp.ogg"),
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.05)),
                mode: PlaybackMode::Once,
                ..Default::default()
            },
        });
    });
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    parent: Query<Entity, With<PigParent>>,
    mut money: ResMut<Money>,
    asset_server: Res<AssetServer>,
) {
    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;

            commands
                .entity(parent.single())
                .remove_children(&[pig_entity]);
            commands.entity(pig_entity).despawn();

            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/pig.ogg"),
                settings: PlaybackSettings {
                    volume: bevy::audio::Volume::Relative(bevy::audio::VolumeLevel::new(0.05)),
                    mode: PlaybackMode::Once,
                    ..Default::default()
                },
            });

            debug!("Pig sold for $15! Current Money: ${:?}", money.0);
        }
    }
}
