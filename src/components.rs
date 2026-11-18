//! Shared ECS components and resources.

use crate::map::TileMap;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Villager {
    pub direction: Vec2,
    pub change_timer: Timer,
}

#[derive(Component)]
pub struct Coin;

/// Axis-aligned collision box (half width, half height).
#[derive(Component, Clone, Copy)]
pub struct Collider(pub Vec2);

#[derive(Component)]
pub struct Speed(pub f32);

/// The loaded level, shared by every system.
#[derive(Resource)]
pub struct Level(pub TileMap);

/// Gold collected by the player.
#[derive(Resource, Default, Debug)]
pub struct Treasury {
    pub gold: u32,
    pub total_coins: u32,
}

/// Draw order (z) for each layer.
pub mod layer {
    pub const GROUND: f32 = 0.0;
    pub const PROPS: f32 = 1.0;
    pub const ITEMS: f32 = 2.0;
    pub const CHARACTERS: f32 = 3.0;
}
