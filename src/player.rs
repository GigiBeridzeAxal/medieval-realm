//! Knight controls: WASD / arrow keys.

use crate::components::*;
use crate::movement::move_and_slide;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

/// Read the keyboard into a direction (not normalised).
pub fn input_direction(keys: &ButtonInput<KeyCode>) -> Vec2 {
    let mut dir = Vec2::ZERO;
    if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        dir.y += 1.0;
    }
    if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        dir.y -= 1.0;
    }
    if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        dir.x -= 1.0;
    }
    if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        dir.x += 1.0;
    }
    dir
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    level: Res<Level>,
    mut player: Single<(&mut Transform, &Collider, &Speed), With<Player>>,
) {
    let (ref mut transform, collider, speed) = *player;
    let dir = input_direction(&keys).normalize_or_zero();
    if dir == Vec2::ZERO {
        return;
    }
    let delta = dir * speed.0 * time.delta_secs();
    let new_pos = move_and_slide(
        &level.0,
        transform.translation.truncate(),
        delta,
        collider.0,
    );
    transform.translation.x = new_pos.x;
    transform.translation.y = new_pos.y;
    // Face the direction of travel.
    if dir.x != 0.0 {
        transform.scale.x = dir.x.signum();
    }
}
