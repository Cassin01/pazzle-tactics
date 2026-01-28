use crate::prelude::*;
use crate::puzzle::TileType;
use super::{Unit, UnitStats, UnitType, HexPosition, BattleGrid, Team, Target, AttackCooldown};

pub fn targeting_system(
    units: Query<(Entity, &HexPosition, &Team), With<Unit>>,
    mut targets: Query<&mut Target, With<Unit>>,
) {
    let unit_data: Vec<(Entity, HexPosition, Team)> = units
        .iter()
        .map(|(e, p, t)| (e, *p, *t))
        .collect();

    for (entity, pos, team) in &unit_data {
        let mut closest: Option<(Entity, i32)> = None;

        for (other_entity, other_pos, other_team) in &unit_data {
            if entity == other_entity || team == other_team {
                continue;
            }

            let dist = pos.distance(other_pos);
            if closest.is_none() || dist < closest.unwrap().1 {
                closest = Some((*other_entity, dist));
            }
        }

        if let Ok(mut target) = targets.get_mut(*entity) {
            target.0 = closest.map(|(e, _)| e);
        }
    }
}

pub fn movement_system(
    _time: Res<Time>,
    grid: Res<BattleGrid>,
    units: Query<(Entity, &HexPosition, &UnitStats, &Target), With<Unit>>,
) {
    let _ = (grid, units);
}

pub fn attack_system(
    time: Res<Time>,
    mut param_set: ParamSet<(
        Query<(&HexPosition, &UnitStats, &Target, &mut AttackCooldown), With<Unit>>,
        Query<&mut UnitStats, With<Unit>>,
    )>,
) {
    let attacks: Vec<(Entity, f32)> = {
        let attackers = param_set.p0();
        attackers
            .iter()
            .filter_map(|(_pos, stats, target, cooldown)| {
                if cooldown.0 <= 0.0 {
                    target.0.map(|t| (t, stats.attack))
                } else {
                    None
                }
            })
            .collect()
    };

    {
        let mut targets = param_set.p1();
        for (target_entity, damage) in attacks {
            if let Ok(mut target_stats) = targets.get_mut(target_entity) {
                target_stats.take_damage(damage);
            }
        }
    }

    {
        let mut attackers = param_set.p0();
        for (_pos, stats, _target, mut cooldown) in attackers.iter_mut() {
            cooldown.0 -= time.delta_secs();
            if cooldown.0 <= 0.0 {
                cooldown.0 = 1.0 / stats.attack_speed;
            }
        }
    }
}

pub fn ability_system(
    mut units: Query<(&mut UnitStats, &UnitType), With<Unit>>,
) {
    for (mut stats, unit_type) in units.iter_mut() {
        if !stats.can_cast() {
            continue;
        }

        match unit_type.0 {
            TileType::Red => {}
            TileType::Blue => {
                let heal = stats.max_health * 0.2;
                stats.health = (stats.health + heal).min(stats.max_health);
            }
            TileType::Purple => {}
            _ => {}
        }

        stats.mana = 0.0;
    }
}

pub fn death_system(
    mut commands: Commands,
    mut grid: ResMut<BattleGrid>,
    units: Query<(Entity, &HexPosition, &UnitStats), With<Unit>>,
) {
    for (entity, pos, stats) in units.iter() {
        if stats.is_dead() {
            grid.remove_unit(pos);
            commands.entity(entity).despawn();
        }
    }
}
