//! Tile collision for moving characters. Pure functions, unit-tested.

use crate::map::{TILE_SIZE, TileMap, world_to_tile};
use bevy::math::Vec2;

/// Does an axis-aligned box (centre `pos`, half extents `half`) overlap any blocked tile?
pub fn box_hits_wall(map: &TileMap, pos: Vec2, half: Vec2) -> bool {
    // Shrink a hair so touching an edge exactly is not a collision.
    let eps = 0.01;
    let (c0, r1) = world_to_tile(pos - half + Vec2::splat(eps));
    let (c1, r0) = world_to_tile(pos + half - Vec2::splat(eps));
    (r0..=r1).any(|r| (c0..=c1).any(|c| !map.is_walkable(c, r)))
}

/// Move a box by `delta`, sliding along walls.
///
/// Each axis is resolved separately, so walking diagonally into a wall slides
/// along it instead of stopping dead. Large deltas are split into sub-steps so
/// a lag spike can never tunnel through a wall.
pub fn move_and_slide(map: &TileMap, mut pos: Vec2, delta: Vec2, half: Vec2) -> Vec2 {
    let max_step = (TILE_SIZE * 0.25).min(half.min_element());
    let steps = (delta.abs().max_element() / max_step).ceil().max(1.0) as u32;
    let step = delta / steps as f32;

    for _ in 0..steps {
        let try_x = Vec2::new(pos.x + step.x, pos.y);
        if !box_hits_wall(map, try_x, half) {
            pos = try_x;
        }
        let try_y = Vec2::new(pos.x, pos.y + step.y);
        if !box_hits_wall(map, try_y, half) {
            pos = try_y;
        }
    }
    pos
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::tile_to_world;

    // 5x5 room, wall in the middle at (2,2).
    const ROOM: &str = "#####\n#.P.#\n#.#.#\n#...#\n#####\n";
    const HALF: Vec2 = Vec2::splat(10.0);

    fn room() -> TileMap {
        TileMap::parse(ROOM).unwrap()
    }

    #[test]
    fn free_movement() {
        let map = room();
        let start = tile_to_world(1, 1);
        let end = move_and_slide(&map, start, Vec2::new(0.0, -20.0), HALF);
        assert!(
            end.abs_diff_eq(start + Vec2::new(0.0, -20.0), 1e-3),
            "{end:?}"
        );
    }

    #[test]
    fn blocked_by_wall() {
        let map = room();
        let start = tile_to_world(2, 1);
        // Straight down into the centre pillar: cannot move.
        let end = move_and_slide(&map, start, Vec2::new(0.0, -20.0), HALF);
        assert!(end.y >= start.y - (TILE_SIZE / 2.0 - HALF.y) - 0.01);
        assert!(!box_hits_wall(&map, end, HALF));
    }

    #[test]
    fn slides_along_wall() {
        let map = room();
        let start = tile_to_world(1, 1);
        // Up-right into the top wall: y is blocked but x still moves.
        let end = move_and_slide(&map, start, Vec2::new(15.0, 15.0), HALF);
        assert!(end.x > start.x + 14.0);
        assert!(!box_hits_wall(&map, end, HALF));
    }

    #[test]
    fn no_tunnelling_on_huge_delta() {
        let map = room();
        let start = tile_to_world(1, 1);
        let end = move_and_slide(&map, start, Vec2::new(-10_000.0, 0.0), HALF);
        assert!(!box_hits_wall(&map, end, HALF));
        assert_eq!(world_to_tile(end), (1, 1));
    }

    #[test]
    fn spawn_point_is_clear() {
        let map = room();
        let (c, r) = map.player_spawn();
        assert!(!box_hits_wall(&map, tile_to_world(c, r), HALF));
    }
}
