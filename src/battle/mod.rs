mod hex_grid;
mod unit;
mod combat;
mod synergy;

use crate::prelude::*;

pub use hex_grid::{BattleGrid, HexPosition};
pub use unit::{Unit, UnitStats, UnitType, StarRank, Team, Target, AttackCooldown};
pub use synergy::{ActiveSynergies, SynergyLevel};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BattleGrid>()
            .init_resource::<ActiveSynergies>()
            .add_systems(Startup, hex_grid::setup_battle_grid)
            .add_systems(
                Update,
                (
                    combat::targeting_system,
                    combat::movement_system,
                    combat::attack_system,
                    combat::ability_system,
                    combat::death_system,
                    synergy::update_synergies,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
