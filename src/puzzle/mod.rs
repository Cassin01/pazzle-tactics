mod board;
mod tile;
mod input;
mod match_detector;
mod cascade;

use crate::prelude::*;

pub use board::PuzzleBoard;
pub use tile::{Tile, TileType, GridPosition, Matched, Falling, Selected};

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, board::setup_puzzle_board)
            .add_observer(input::handle_tile_swap)
            .add_systems(
                Update,
                (
                    input::handle_tile_click,
                    match_detector::detect_matches,
                    match_detector::remove_matched_tiles,
                    cascade::apply_gravity,
                    cascade::spawn_new_tiles,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
