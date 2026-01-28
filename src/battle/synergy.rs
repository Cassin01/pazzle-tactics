use crate::prelude::*;
use crate::puzzle::TileType;
use super::{Unit, UnitType, UnitStats, Team};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SynergyLevel {
    None,
    Bronze,
    Silver,
    Gold,
}

impl SynergyLevel {
    pub fn from_count(count: usize) -> Self {
        match count {
            0..=1 => SynergyLevel::None,
            2..=3 => SynergyLevel::Bronze,
            4..=5 => SynergyLevel::Silver,
            _ => SynergyLevel::Gold,
        }
    }

    pub fn bonus_multiplier(&self) -> f32 {
        match self {
            SynergyLevel::None => 1.0,
            SynergyLevel::Bronze => 1.15,
            SynergyLevel::Silver => 1.30,
            SynergyLevel::Gold => 1.50,
        }
    }
}

#[derive(Resource, Default)]
pub struct ActiveSynergies {
    pub bonuses: HashMap<TileType, SynergyLevel>,
}

impl ActiveSynergies {
    pub fn get_level(&self, tile_type: TileType) -> SynergyLevel {
        self.bonuses.get(&tile_type).copied().unwrap_or(SynergyLevel::None)
    }
}

pub fn update_synergies(
    mut synergies: ResMut<ActiveSynergies>,
    units: Query<(&UnitType, &Team), With<Unit>>,
) {
    let mut counts: HashMap<TileType, usize> = HashMap::new();

    for (unit_type, team) in units.iter() {
        if *team == Team::Player {
            *counts.entry(unit_type.0).or_insert(0) += 1;
        }
    }

    synergies.bonuses.clear();
    for (tile_type, count) in counts {
        let level = SynergyLevel::from_count(count);
        if level != SynergyLevel::None {
            synergies.bonuses.insert(tile_type, level);
        }
    }
}

pub fn apply_synergy_bonuses(
    synergies: Res<ActiveSynergies>,
    mut units: Query<(&UnitType, &mut UnitStats, &Team), With<Unit>>,
) {
    for (unit_type, mut stats, team) in units.iter_mut() {
        if *team != Team::Player {
            continue;
        }

        let level = synergies.get_level(unit_type.0);
        let multiplier = level.bonus_multiplier();

        stats.attack *= multiplier;
    }
}
