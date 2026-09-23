use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum HeroState {
    Idle,
    Walk,
    Run,
    Punch,
}
