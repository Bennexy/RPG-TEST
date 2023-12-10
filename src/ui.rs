use crate::game_plugins::player::Player;
use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use rpg_game::game_plugins::world_map::utils::{world_to_chunks, world_to_chunks_tile, world_to_tiles};

// use crate::pig::PigCount;
// use crate::{GameState, Money};
pub struct GameUI;

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
struct FpsRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct PlayerCordsRoot;

#[derive(Component)]
struct PlayerCordsText;

#[derive(Component)]
struct PlayerChunksRoot;
#[derive(Component)]
struct PlayerChunksText;

#[derive(Component)]
struct PlayerTilesRoot;
#[derive(Component)]
struct PlayerTilesText;

#[derive(Component)]
struct PlayerChunkTilesRoot;
#[derive(Component)]
struct PlayerChunkTilesText;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(
                Startup,
                (
                    setup_fps_counter,
                    setup_player_cords,
                    setup_player_chunks,
                    setup_player_tiles,
                    setup_player_chunk_tiles,
                ),
            )
            .add_systems(
                Update,
                (
                    fps_text_update_system,
                    player_cords_text_update_system,
                    fps_counter_showhide,
                ),
            );
    }
}

fn setup_player_tiles(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            PlayerTilesRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(16.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_cords = commands
        .spawn((
            PlayerTilesText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "PlayerTiles: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(root).push_children(&[text_cords]);
}

fn setup_player_chunks(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            PlayerChunksRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(11.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    // create our text
    let text_chunks = commands
        .spawn((
            PlayerChunksText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "PlayerChunks: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_chunks]);
}

fn setup_player_cords(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            PlayerCordsRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(6.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_cords = commands
        .spawn((
            PlayerCordsText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "PlayerCords: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(root).push_children(&[text_cords]);
}

fn setup_player_chunk_tiles(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            PlayerChunkTilesRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(21.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_cords = commands
        .spawn((
            PlayerChunkTilesText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "PlayerChunk-tiles: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();

    commands.entity(root).push_children(&[text_cords]);
}

fn player_cords_text_update_system(
    player_cords: Query<&Transform, With<Player>>,
    mut query: Query<
        &mut Text,
        (
            With<PlayerCordsText>,
            Without<PlayerChunksText>,
            Without<PlayerTilesText>,
            Without<PlayerChunkTilesText>,
        ),
    >,
    mut query_chunks: Query<
        &mut Text,
        (
            With<PlayerChunksText>,
            Without<PlayerCordsText>,
            Without<PlayerTilesText>,
            Without<PlayerChunkTilesText>,
        ),
    >,
    mut query_tiles: Query<
        &mut Text,
        (
            With<PlayerTilesText>,
            Without<PlayerCordsText>,
            Without<PlayerChunksText>,
            Without<PlayerChunkTilesText>,
        ),
    >,

    mut query_chunk_tiles: Query<
        &mut Text,
        (
            With<PlayerChunkTilesText>,
            Without<PlayerCordsText>,
            Without<PlayerChunksText>,
            Without<PlayerTilesText>,
        ),
    >,
) {
    let player_cords = player_cords.single();
    let mut text = query.single_mut();
    text.sections[1].value = format!(
        "x: {:.2}, y: {:.2}",
        player_cords.translation.x, player_cords.translation.y
    )
    .into();
    text.sections[1].style.color = Color::WHITE;

    let mut text = query_chunks.single_mut();
    let (x, y) = world_to_chunks((player_cords.translation.x, player_cords.translation.y));
    text.sections[1].value = format!("x: {:.2}, y: {:.2}", x, y).into();
    text.sections[1].style.color = Color::WHITE;

    let mut text = query_tiles.single_mut();
    let (x, y) = world_to_tiles((player_cords.translation.x, player_cords.translation.y));
    text.sections[1].value = format!("x: {:.2}, y: {:.2}", x, y).into();
    text.sections[1].style.color = Color::WHITE;

    let mut text = query_chunk_tiles.single_mut();
    let (x, y) = world_to_chunks_tile((player_cords.translation.x, player_cords.translation.y));
    text.sections[1].value = format!("x: {}, y: {:?}", x, y).into();
    text.sections[1].style.color = Color::WHITE;
}

fn setup_fps_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            FpsRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Percent(1.),
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Auto,
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_fps = commands
        .spawn((
            FpsText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "FPS: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

fn fps_text_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy
        if let Some(value) = diagnostics
            .get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            // Format the number as to leave space for 4 digits, just in case,
            // right-aligned and rounded. This helps readability when the
            // number changes rapidly.
            text.sections[1].value = format!("{value:>4.0}");

            // Let's make it extra fancy by changing the color of the
            // text according to the FPS value:
            text.sections[1].style.color = if value >= 120.0 {
                // Above 120 FPS, use green color
                Color::rgb(0.0, 1.0, 0.0)
            } else if value >= 60.0 {
                // Between 60-120 FPS, gradually transition from yellow to green
                Color::rgb((1.0 - (value - 60.0) / (120.0 - 60.0)) as f32, 1.0, 0.0)
            } else if value >= 30.0 {
                // Between 30-60 FPS, gradually transition from red to yellow
                Color::rgb(1.0, ((value - 30.0) / (60.0 - 30.0)) as f32, 0.0)
            } else {
                // Below 30 FPS, use red color
                Color::rgb(1.0, 0.0, 0.0)
            }
        } else {
            // display "N/A" if we can't get a FPS measurement
            // add an extra space to preserve alignment
            text.sections[1].value = " N/A".into();
            text.sections[1].style.color = Color::WHITE;
        }
    }
}

/// Toggle the FPS counter when pressing F12
fn fps_counter_showhide(
    mut fps_root: Query<
        &mut Visibility,
        (
            With<FpsRoot>,
            Without<PlayerCordsRoot>,
            Without<PlayerChunksRoot>,
            Without<PlayerTilesRoot>,
            Without<PlayerChunkTilesRoot>,
        ),
    >,

    mut player_cords_root: Query<
        &mut Visibility,
        (
            With<PlayerCordsRoot>,
            Without<PlayerChunksRoot>,
            Without<PlayerTilesRoot>,
            Without<PlayerChunkTilesRoot>,
        ),
    >,
    mut query_chunks: Query<
        &mut Visibility,
        (
            With<PlayerChunksRoot>,
            Without<PlayerCordsRoot>,
            Without<PlayerTilesRoot>,
            Without<PlayerChunkTilesRoot>,
        ),
    >,
    mut query_tiles: Query<
        &mut Visibility,
        (
            With<PlayerTilesRoot>,
            Without<PlayerCordsRoot>,
            Without<PlayerChunksRoot>,
            Without<PlayerChunkTilesRoot>,
        ),
    >,

    mut query_chunk_tiles: Query<
        &mut Visibility,
        (
            With<PlayerChunkTilesRoot>,
            Without<PlayerCordsRoot>,
            Without<PlayerChunksRoot>,
            Without<PlayerTilesRoot>,
        ),
    >,
    kbd: Res<Input<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        let mut vis = fps_root.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
        let mut vis = query_chunks.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };

        let mut vis = player_cords_root.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };

        let mut vis = query_chunk_tiles.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };

        let mut vis = query_tiles.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };


        // for element in vec![
        //     query_chunks, player_cords_root, fps_root, query_chunk_tiles, query_tiles
        // ].iter() {
        //     let mut vis = element.single_mut();
        //     *vis = match *vis {
        //         Visibility::Hidden => Visibility::Visible,
        //         _ => Visibility::Hidden,
        //     };
        // }

    }
}
