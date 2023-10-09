use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Controller {
    pub direction: Vec3,
}

#[derive(Component, Debug)]
pub struct IsFalling {
    pub last_y: f32,
    pub falling: bool,
}

impl IsFalling {
    pub fn default() -> Self {
        Self {
            last_y: 0.0,
            falling: false,
        }
    }
}

#[derive(Component)]
pub struct Animator(pub usize, pub usize);

#[derive(Component)]
pub struct Binder; // can only inserted once

#[derive(Component)]
pub struct BinderFollwer(pub Transform); // can only inserted once
