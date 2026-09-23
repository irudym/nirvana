use bevy::prelude::*;
use bevy::{
    asset::{Assets, RenderAssetUsages},
    camera::{RenderTarget, ScalingMode, visibility::RenderLayers},
    image::ImageSampler,
    render::render_resource::{Extent3d, TextureDimension, TextureUsages},
};

const W_RES: u32 = 640;
const H_RES: u32 = 360;
const VIEWPORT_HEIGHT: f32 = 6.0;
const CAMERA_Y: f32 = VIEWPORT_HEIGHT / 2.0 - 0.5; // floor sits 0.5 m above the bottom edge

#[derive(Component)]
pub struct UpscaleSprite;

pub fn spawn_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: W_RES,
        height: H_RES,
        depth_or_array_layers: 1,
    };
    let mut img = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        bevy::render::render_resource::TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    img.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    img.sampler = ImageSampler::nearest();
    let target = images.add(img);

    let viewport_height = VIEWPORT_HEIGHT; //16.0;
    let camera_y = CAMERA_Y; // floor sits 0.5 m above the bottom edge

    commands.spawn((
        Camera2d,
        RenderTarget::Image(target.clone().into()),
        Camera {
            order: -1, //draw first

            clear_color: ClearColorConfig::Custom(Color::srgb_u8(20, 18, 30)),
            ..default()
        },
        Msaa::Off,
        RenderLayers::layer(2),
    ));

    // 3D side-view camera
    commands.spawn((
        Camera3d::default(),
        RenderTarget::Image(target.clone().into()),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: viewport_height,
            },
            ..OrthographicProjection::default_3d()
        }),
        Msaa::Off,
        Transform::from_xyz(0.0, camera_y, 10.0).looking_at(Vec3::new(0.0, camera_y, 0.0), Vec3::Y),
    ));

    //2D camera that shows the texture on screen
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::srgb_u8(200, 200, 210)),
            ..default()
        },
        Msaa::Off,
        RenderLayers::layer(1),
    ));
    commands.spawn((
        Sprite::from_image(target),
        UpscaleSprite,
        RenderLayers::layer(1),
    ));
}

// Scale the image by the largest whole number that fits the window
pub fn fit_to_window(windows: Query<&Window>, mut q: Query<&mut Transform, With<UpscaleSprite>>) {
    let Ok(w) = windows.single() else { return };
    let s = (w.width() / W_RES as f32)
        .min(w.height() / H_RES as f32)
        .floor()
        .max(1.0);
    for mut t in &mut q {
        t.scale = Vec3::splat(s);
    }
}
