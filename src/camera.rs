//! Camera smoothly follows the knight and never shows outside the map.

use crate::components::*;
use bevy::prelude::*;

const FOLLOW_SPEED: f32 = 6.0;
const ZOOM: f32 = 0.6; // < 1 zooms in, for a chunky pixel look

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera).add_systems(
            PostUpdate,
            follow_player.before(TransformSystems::Propagate),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: ZOOM,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

/// Clamp a camera centre so the view stays inside the world bounds.
/// If the view is bigger than the world on an axis, centre it.
pub fn clamp_camera(target: Vec2, half_view: Vec2, min: Vec2, max: Vec2) -> Vec2 {
    let clamp_axis = |t: f32, h: f32, lo: f32, hi: f32| {
        if hi - lo <= 2.0 * h {
            (lo + hi) / 2.0
        } else {
            t.clamp(lo + h, hi - h)
        }
    };
    Vec2::new(
        clamp_axis(target.x, half_view.x, min.x, max.x),
        clamp_axis(target.y, half_view.y, min.y, max.y),
    )
}

fn follow_player(
    time: Res<Time>,
    level: Res<Level>,
    window: Single<&Window>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let half_view = Vec2::new(window.width(), window.height()) * ZOOM / 2.0;
    let (min, max) = level.0.world_bounds();
    let goal = clamp_camera(player.translation.truncate(), half_view, min, max);
    let mut pos = camera.translation.truncate();
    pos.smooth_nudge(&goal, FOLLOW_SPEED, time.delta_secs());
    camera.translation.x = pos.x;
    camera.translation.y = pos.y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_inside_world() {
        let (min, max) = (Vec2::new(0.0, 0.0), Vec2::new(1000.0, 500.0));
        let half = Vec2::new(100.0, 50.0);
        assert_eq!(
            clamp_camera(Vec2::new(-50.0, -50.0), half, min, max),
            Vec2::new(100.0, 50.0)
        );
        assert_eq!(
            clamp_camera(Vec2::new(5000.0, 5000.0), half, min, max),
            Vec2::new(900.0, 450.0)
        );
        assert_eq!(
            clamp_camera(Vec2::new(300.0, 200.0), half, min, max),
            Vec2::new(300.0, 200.0)
        );
    }

    #[test]
    fn centres_when_view_is_larger_than_world() {
        let (min, max) = (Vec2::ZERO, Vec2::new(100.0, 100.0));
        let got = clamp_camera(Vec2::new(80.0, 10.0), Vec2::splat(500.0), min, max);
        assert_eq!(got, Vec2::new(50.0, 50.0));
    }
}
