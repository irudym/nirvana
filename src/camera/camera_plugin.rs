use crate::camera::camera::{fit_to_window, spawn_camera};
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, fit_to_window);
    }
}
