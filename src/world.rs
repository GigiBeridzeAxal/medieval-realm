//! Builds the level from the map file: terrain tiles, then characters and items.
//!
//! Art is placeholder coloured shapes so the project runs with zero asset files.
//! Swap `Sprite::from_color` for `Sprite::from_image` when real art arrives.

use crate::components::*;
use crate::map::{SpawnMarker, TILE_SIZE, TileKind, TileMap, tile_to_world};
use bevy::prelude::*;

pub const LEVEL_FILE: &str = include_str!("../assets/maps/village.txt");

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let map = TileMap::parse(LEVEL_FILE).unwrap_or_else(|e| panic!("bad level file: {e}"));
        app.insert_resource(Level(map))
            .insert_resource(ClearColor(Color::srgb(0.05, 0.08, 0.05)))
            .add_systems(Startup, spawn_level);
    }
}

fn tile_color(kind: TileKind) -> Color {
    match kind {
        TileKind::Grass => Color::srgb(0.30, 0.55, 0.25),
        TileKind::Path => Color::srgb(0.66, 0.55, 0.36),
        TileKind::Water => Color::srgb(0.18, 0.38, 0.70),
        TileKind::Bridge => Color::srgb(0.48, 0.32, 0.18),
        TileKind::CastleWall => Color::srgb(0.42, 0.42, 0.46),
        TileKind::CastleFloor => Color::srgb(0.62, 0.60, 0.56),
        TileKind::Gate => Color::srgb(0.30, 0.20, 0.12),
        // Props are drawn on top of grass, see spawn_prop.
        TileKind::Tree | TileKind::House => Color::srgb(0.30, 0.55, 0.25),
    }
}

fn spawn_level(mut commands: Commands, level: Res<Level>, mut treasury: ResMut<Treasury>) {
    let map = &level.0;
    let tile = Vec2::splat(TILE_SIZE);

    for (col, row, kind) in map.iter() {
        let pos = tile_to_world(col, row);
        commands.spawn((
            Sprite::from_color(tile_color(kind), tile),
            Transform::from_translation(pos.extend(layer::GROUND)),
        ));
        spawn_prop(&mut commands, kind, pos);
    }

    for &(marker, col, row) in &map.spawns {
        let pos = tile_to_world(col, row);
        match marker {
            SpawnMarker::Player => spawn_knight(&mut commands, pos),
            SpawnMarker::Villager => spawn_villager(&mut commands, pos),
            SpawnMarker::Coin => {
                spawn_coin(&mut commands, pos);
                treasury.total_coins += 1;
            }
        }
    }
}

fn spawn_prop(commands: &mut Commands, kind: TileKind, pos: Vec2) {
    let z = layer::PROPS;
    match kind {
        TileKind::Tree => {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.36, 0.24, 0.12), Vec2::new(6.0, 12.0)),
                Transform::from_xyz(pos.x, pos.y - 9.0, z),
            ));
            commands.spawn((
                Sprite::from_color(Color::srgb(0.12, 0.35, 0.15), Vec2::new(26.0, 22.0)),
                Transform::from_xyz(pos.x, pos.y + 4.0, z + 0.01),
            ));
        }
        TileKind::House => {
            commands.spawn((
                Sprite::from_color(Color::srgb(0.85, 0.78, 0.62), Vec2::new(30.0, 18.0)),
                Transform::from_xyz(pos.x, pos.y - 6.0, z),
            ));
            commands.spawn((
                Sprite::from_color(Color::srgb(0.60, 0.22, 0.15), Vec2::new(32.0, 14.0)),
                Transform::from_xyz(pos.x, pos.y + 9.0, z + 0.01),
            ));
        }
        TileKind::CastleWall => {
            // Crenellation on top of the wall.
            commands.spawn((
                Sprite::from_color(Color::srgb(0.32, 0.32, 0.36), Vec2::new(10.0, 10.0)),
                Transform::from_xyz(pos.x, pos.y + 6.0, z),
            ));
        }
        _ => {}
    }
}

pub fn spawn_knight(commands: &mut Commands, pos: Vec2) {
    commands.spawn((
        Player,
        Collider(Vec2::new(9.0, 11.0)),
        Speed(140.0),
        Transform::from_translation(pos.extend(layer::CHARACTERS)),
        Visibility::default(),
        children![
            // Steel armour body
            (
                Sprite::from_color(Color::srgb(0.70, 0.72, 0.78), Vec2::new(18.0, 20.0)),
                Transform::from_xyz(0.0, -2.0, 0.0)
            ),
            // Red tabard with the house colours
            (
                Sprite::from_color(Color::srgb(0.75, 0.10, 0.12), Vec2::new(10.0, 14.0)),
                Transform::from_xyz(0.0, -3.0, 0.1)
            ),
            // Helmet
            (
                Sprite::from_color(Color::srgb(0.55, 0.57, 0.62), Vec2::new(14.0, 10.0)),
                Transform::from_xyz(0.0, 11.0, 0.1)
            ),
            // Sword
            (
                Sprite::from_color(Color::srgb(0.90, 0.90, 0.95), Vec2::new(3.0, 18.0)),
                Transform::from_xyz(12.0, 0.0, 0.2)
            ),
        ],
    ));
}

pub fn spawn_villager(commands: &mut Commands, pos: Vec2) {
    commands.spawn((
        Villager {
            direction: Vec2::ZERO,
            change_timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
        Collider(Vec2::new(8.0, 10.0)),
        Speed(45.0),
        Transform::from_translation(pos.extend(layer::CHARACTERS)),
        Visibility::default(),
        children![
            (
                Sprite::from_color(Color::srgb(0.45, 0.30, 0.18), Vec2::new(16.0, 18.0)),
                Transform::from_xyz(0.0, -3.0, 0.0)
            ),
            (
                Sprite::from_color(Color::srgb(0.93, 0.78, 0.62), Vec2::new(11.0, 10.0)),
                Transform::from_xyz(0.0, 10.0, 0.1)
            ),
        ],
    ));
}

pub fn spawn_coin(commands: &mut Commands, pos: Vec2) {
    commands.spawn((
        Coin,
        Sprite::from_color(Color::srgb(1.0, 0.82, 0.15), Vec2::splat(12.0)),
        Transform::from_translation(pos.extend(layer::ITEMS))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
    ));
}
