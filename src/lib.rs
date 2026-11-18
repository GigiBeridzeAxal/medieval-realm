//! Medieval Realm: a 2D top-down medieval game built with Rust + Bevy.
//!
//! Each feature lives in its own plugin. Pure game logic (map, movement, rng)
//! has no Bevy rendering dependencies and is unit-tested.

pub mod camera;
pub mod components;
pub mod hud;
pub mod map;
pub mod movement;
pub mod player;
pub mod rng;
pub mod treasure;
pub mod villager;
pub mod world;

use bevy::prelude::*;

/// Everything gameplay-related. Add this to an App with `DefaultPlugins`.
pub struct MedievalGamePlugin;

impl Plugin for MedievalGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            treasure::TreasurePlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
            villager::VillagerPlugin,
            camera::CameraPlugin,
            hud::HudPlugin,
        ));
    }
}
