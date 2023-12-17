use bevy::prelude::Component;

#[derive(Component, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Player;
