#import bevy_pbr::forward_io::VertexOutput

struct PlaneShadeMaterial {
    base_color: vec4<f32>,
    shadow_tint: vec4<f32>,
    plane_z: f32,
    threshold: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material: PlaneShadeMaterial;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var color_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = material.base_color;

#ifdef VERTEX_UVS_A
    color = color * textureSample(color_texture, color_sampler, in.uv);
#endif

    if in.world_position.z < material.plane_z - material.threshold {
        color = vec4(color.rgb * material.shadow_tint.rgb, color.a);
    }

    return color;
}
