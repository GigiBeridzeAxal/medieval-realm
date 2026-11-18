//! Villagers wander around the map randomly.

use crate::components::*;
use crate::movement::move_and_slide;
use crate::rng::GameRng;
use bevy::prelude::*;

pub struct VillagerPlugin;

impl Plugin for VillagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameRng::new(0x5EED_CA57))
            .add_systems(Update, wander);
    }
}

pub fn wander(
    time: Res<Time>,
    level: Res<Level>,
    mut rng: ResMut<GameRng>,
    mut villagers: Query<(&mut Villager, &mut Transform, &Collider, &Speed)>,
) {
    for (mut villager, mut transform, collider, speed) in &mut villagers {
        if villager.change_timer.tick(time.delta()).is_finished() {
            // 30% chance to stand still and chat, otherwise pick a new heading.
            villager.direction = if rng.next_f32() < 0.3 {
                Vec2::ZERO
            } else {
                Vec2::from_angle(rng.range(0.0, std::f32::consts::TAU))
            };
            villager.change_timer = Timer::from_seconds(rng.range(1.0, 3.5), TimerMode::Once);
        }

        let start = transform.translation.truncate();
        let delta = villager.direction * speed.0 * time.delta_secs();
        let end = move_and_slide(&level.0, start, delta, collider.0);
        // Bumped into something: turn around next frame.
        if delta != Vec2::ZERO && end.distance(start) < delta.length() * 0.5 {
            villager.direction = -villager.direction;
        }
        transform.translation.x = end.x;
        transform.translation.y = end.y;
    }
}
