use bevy::prelude::*;

use crate::Money;
use crate::pig::PigCount;
pub struct GameUI;

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct PigParentText;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, (update_money_ui, update_pig_count_ui));
    }
}

fn spawn_game_ui(mut commands: Commands, asset_loader: Res<AssetServer>) {
    let font = asset_loader.load("Dragon_Fire_font.otf");
    let font2 = asset_loader.load("Dragon_Fire_font.otf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    style: Style { margin: UiRect { left: Val::Px(50.), ..default() }, ..default()},
                    text: Text::from_section(
                        "Money!",
                        TextStyle {
                            font,
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                MoneyText,
            ));
        })
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    style: Style { margin: UiRect { left: Val::Px(50.), ..default() }, ..default()},
                    text: Text::from_section(
                        "Pigs Spawned!",
                        TextStyle {
                            font: font2,
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                },
                PigParentText,
            ));
        });
}

fn update_money_ui(mut texts: Query<&mut Text, With<MoneyText>>, money: Res<Money>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Money: ${:?}", money.0);
    }
}
fn update_pig_count_ui(
    mut texts: Query<&mut Text, With<PigParentText>>,
    pig_count: Res<PigCount>,
) {

    for mut text in &mut texts {
        text.sections[0].value = format!("Pigs Spawned: {:?}", pig_count.0);
    }
}
