use bevy::prelude::*;

#[derive(Resource)]
pub struct HeroAnimations {
    pub graph: Handle<AnimationGraph>,
    pub idle: AnimationNodeIndex,
    pub walk: AnimationNodeIndex,
    pub run: AnimationNodeIndex,
    pub punch: AnimationNodeIndex,
}

#[derive(Component)]
pub struct HeroAnimPlayer(pub Entity);
