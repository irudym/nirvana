use bevy::prelude::*;
use nirvana::{camera::CameraPlugin, materials::PlaneShadeMaterial, world::WorldPlugin};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Nirvana".into(),
                    resolution: (1280, 720).into(),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(MaterialPlugin::<PlaneShadeMaterial>::default())
    .add_plugins((CameraPlugin, WorldPlugin));

    app.run();
}
