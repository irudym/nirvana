use crate::{
    character::hero::hero_control, materials::plane_shade_material::update_plane_shading,
    world::world::spawn_world,
};
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world)
            .add_systems(Update, (hero_control, update_plane_shading));
    }
}
