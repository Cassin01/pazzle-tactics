use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct BattleCamera;

#[derive(Component)]
pub struct PuzzleCamera;

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera,
    ));
}
