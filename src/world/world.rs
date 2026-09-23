use bevy::prelude::*;

use crate::character::spawn_hero;

pub fn spawn_world(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    spawn_hero(commands, assets, graphs);
}
