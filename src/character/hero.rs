use crate::character::hero_animations::{HeroAnimPlayer, HeroAnimations};
use crate::character::hero_state::HeroState;
use crate::materials::{PlaneShadeMaterial, PlaneShading};
use bevy::prelude::*;
use bevy::{image::ImageSampler, world_serialization::WorldInstanceReady};
use std::collections::HashMap;
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Facing {
    pub yaw: f32,
    pub target: f32,
}

pub fn spawn_hero(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let clip = |i| assets.load(GltfAssetLabel::Animation(i).from_asset("models/man2.glb"));
    let (graph, nodes) = AnimationGraph::from_clips([clip(0), clip(3), clip(2), clip(1)]);
    commands.insert_resource(HeroAnimations {
        graph: graphs.add(graph),
        idle: nodes[0],
        walk: nodes[1],
        run: nodes[2],
        punch: nodes[3],
    });

    commands
        .spawn((
            WorldAssetRoot(assets.load(GltfAssetLabel::Scene(0).from_asset("models/man2.glb"))),
            Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
            Hero,
            HeroState::Idle,
            Facing {
                yaw: FRAC_PI_2,
                target: FRAC_PI_2,
            },
        ))
        .observe(on_hero_ready)
        //observe(apply_side_shading);
        .observe(apply_plane_shading);
}

fn on_hero_ready(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    mut players: Query<&mut AnimationPlayer>,
    anims: Res<HeroAnimations>,
) {
    for e in children.iter_descendants(ready.entity) {
        if let Ok(mut player) = players.get_mut(e) {
            let mut transitions = AnimationTransitions::new();
            transitions
                .play(&mut player, anims.idle, std::time::Duration::ZERO)
                .repeat();
            commands
                .entity(e)
                .insert((AnimationGraphHandle(anims.graph.clone()), transitions));
            commands.entity(ready.entity).insert(HeroAnimPlayer(e));
        }
    }
}

fn apply_plane_shading(
    ready: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    mesh_q: Query<&MeshMaterial3d<StandardMaterial>>,
    std_mats: Res<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut mats: ResMut<Assets<PlaneShadeMaterial>>,
) {
    let mut converted: HashMap<AssetId<StandardMaterial>, Handle<PlaneShadeMaterial>> =
        HashMap::new();

    for e in children.iter_descendants(ready.entity) {
        let Ok(std_handle) = mesh_q.get(e) else {
            continue;
        };
        let handle = converted
            .entry(std_handle.0.id())
            .or_insert_with(|| {
                let (base_color, texture) = std_mats
                    .get(&std_handle.0)
                    .map(|m| (m.base_color.to_linear(), m.base_color_texture.clone()))
                    .unwrap_or((LinearRgba::WHITE, None));

                if let Some(tex) = &texture {
                    if let Some(mut img) = images.get_mut(tex) {
                        img.sampler = ImageSampler::nearest();
                    }
                }

                mats.add(PlaneShadeMaterial {
                    shadow_tint: LinearRgba::rgb(0.30, 0.28, 0.40),
                    base_color,
                    plane_z: 0.0,
                    threshold: 0.0,
                    color_texture: texture,
                })
            })
            .clone();

        commands
            .entity(e)
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert(MeshMaterial3d(handle));
    }

    let handles: Vec<_> = converted.into_values().collect();
    commands.entity(ready.entity).insert(PlaneShading(handles));
}

pub fn hero_control(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    anims: Res<HeroAnimations>,
    mut hero: Query<(&mut HeroState, &mut Transform, &HeroAnimPlayer, &mut Facing), With<Hero>>,
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    let Ok((mut state, mut tf, anim_player, mut facing)) = hero.single_mut() else {
        return;
    };
    let dir = keys.pressed(KeyCode::ArrowRight) as i32 - keys.pressed(KeyCode::ArrowLeft) as i32;
    let running = keys.pressed(KeyCode::ShiftLeft);
    let punch = keys.pressed(KeyCode::Space);

    let next = match dir {
        0 if punch => HeroState::Punch,
        0 => HeroState::Idle,
        _ if running => HeroState::Run,
        _ => HeroState::Walk,
    };
    let speed = match *state {
        HeroState::Walk => 1.0,
        HeroState::Run => 2.8,
        HeroState::Idle => 0.0,
        HeroState::Punch => 0.0,
    };

    if dir != 0 {
        //let facing = if dir > 0 { 1.0 } else { -1.0 };
        facing.target = dir as f32 * std::f32::consts::FRAC_PI_2;

        //tf.rotation = Quat::from_rotation_y(facing * std::f32::consts::FRAC_PI_2);
        tf.translation.x += dir as f32 * speed * time.delta_secs();
    }
    let turn_speed = 7.0; // radians per second
    let step = turn_speed * time.delta_secs();
    facing.yaw += (facing.target - facing.yaw).clamp(-step, step);
    tf.rotation = Quat::from_rotation_y(facing.yaw);

    if next != *state {
        let node = match next {
            HeroState::Idle => anims.idle,
            HeroState::Walk => anims.walk,
            HeroState::Run => anims.run,
            HeroState::Punch => anims.punch,
        };
        if let Ok((mut player, mut transitions)) = players.get_mut(anim_player.0) {
            transitions
                .play(&mut player, node, std::time::Duration::from_millis(150))
                .repeat();
        }
        *state = next;
    }
}
