use crate::prelude::*;
use crate::battle::{ActiveSynergies, SynergyLevel};
use crate::puzzle::TileType;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct SynergyDisplay;

pub fn setup_hud(mut commands: Commands) {
    commands.insert_resource(Score::default());

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Score: 0"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                ScoreText,
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            SynergyDisplay,
        ));
}

pub fn update_score_display(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if score.is_changed() {
        for mut text in query.iter_mut() {
            **text = format!("Score: {}", score.0);
        }
    }
}

pub fn update_synergy_display(
    synergies: Res<ActiveSynergies>,
    mut commands: Commands,
    display: Query<Entity, With<SynergyDisplay>>,
) {
    if !synergies.is_changed() {
        return;
    }

    let Ok(display_entity) = display.get_single() else { return };

    commands.entity(display_entity).despawn_descendants();

    commands.entity(display_entity).with_children(|parent| {
        for tile_type in [TileType::Red, TileType::Blue, TileType::Green, TileType::Yellow, TileType::Purple] {
            let level = synergies.get_level(tile_type);
            if level == SynergyLevel::None {
                continue;
            }

            let level_text = match level {
                SynergyLevel::Bronze => "Bronze",
                SynergyLevel::Silver => "Silver",
                SynergyLevel::Gold => "Gold",
                SynergyLevel::None => continue,
            };

            let type_name = match tile_type {
                TileType::Red => "Warrior",
                TileType::Blue => "Tank",
                TileType::Green => "Ranger",
                TileType::Yellow => "Assassin",
                TileType::Purple => "Mage",
            };

            parent.spawn((
                Text::new(format!("{}: {}", type_name, level_text)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(tile_type.color()),
            ));
        }
    });
}
