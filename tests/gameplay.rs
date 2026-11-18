//! Headless ECS tests: run real game systems without a window or GPU.

use bevy::prelude::*;
use medieval_realm::components::*;
use medieval_realm::map::{TileMap, tile_to_world};
use medieval_realm::treasure::collect_coins;
use medieval_realm::world::{LEVEL_FILE, spawn_coin, spawn_knight};

fn app_with_level(map: &str) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(Level(TileMap::parse(map).unwrap()))
        .init_resource::<Treasury>();
    app
}

#[test]
fn knight_picks_up_nearby_coin_only() {
    let mut app = app_with_level("....\n.P..\n....\n");
    app.add_systems(Update, collect_coins);
    {
        let mut commands = app.world_mut().commands();
        spawn_knight(&mut commands, tile_to_world(1, 1));
        spawn_coin(&mut commands, tile_to_world(1, 1) + Vec2::new(5.0, 0.0)); // close
        spawn_coin(&mut commands, tile_to_world(3, 2)); // far
    }
    app.world_mut().flush();
    app.update();

    assert_eq!(app.world().resource::<Treasury>().gold, 10);
    let left = app
        .world_mut()
        .query_filtered::<(), With<Coin>>()
        .iter(app.world())
        .count();
    assert_eq!(left, 1);
}

#[test]
fn villagers_never_walk_into_walls() {
    use bevy::time::TimeUpdateStrategy;
    use medieval_realm::movement::box_hits_wall;
    use medieval_realm::rng::GameRng;
    use medieval_realm::villager::wander;
    use std::time::Duration;

    let mut app = app_with_level(LEVEL_FILE);
    app.insert_resource(GameRng::new(1234))
        .add_systems(Update, wander);
    let map = TileMap::parse(LEVEL_FILE).unwrap();
    {
        let mut commands = app.world_mut().commands();
        for &(marker, c, r) in &map.spawns {
            if marker == medieval_realm::map::SpawnMarker::Villager {
                medieval_realm::world::spawn_villager(&mut commands, tile_to_world(c, r));
            }
        }
    }
    app.world_mut().flush();

    // Simulate ~60 seconds at 60 FPS using a manual clock.
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f32(
        1.0 / 60.0,
    )));
    for _ in 0..3600 {
        app.update();
    }

    let mut q = app
        .world_mut()
        .query::<(&Transform, &Collider, &Villager)>();
    let mut count = 0;
    for (t, c, _) in q.iter(app.world()) {
        assert!(
            !box_hits_wall(&map, t.translation.truncate(), c.0),
            "villager stuck in wall at {:?}",
            t.translation
        );
        count += 1;
    }
    assert_eq!(count, 3);
}
