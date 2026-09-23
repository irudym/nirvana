use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct PlaneShadeMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(0)]
    pub shadow_tint: LinearRgba,
    #[uniform(0)]
    pub plane_z: f32,
    #[uniform(0)]
    pub threshold: f32,
    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Option<Handle<Image>>,
}

#[derive(Component)]
pub struct PlaneShading(pub Vec<Handle<PlaneShadeMaterial>>);

impl Material for PlaneShadeMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/plane_shade.wgsl".into()
    }
}

pub fn update_plane_shading(
    chars: Query<(&GlobalTransform, &PlaneShading)>,
    mut mats: ResMut<Assets<PlaneShadeMaterial>>,
) {
    for (gt, shading) in &chars {
        let z = gt.translation().z;
        for h in &shading.0 {
            if let Some(mut m) = mats.get_mut(h) {
                if (m.plane_z - z).abs() > 1e-4 {
                    m.plane_z = z;
                }
            }
        }
    }
}
