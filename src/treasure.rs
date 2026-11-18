//! Gold coins: pick up by walking over them.

use crate::components::*;
use bevy::prelude::*;

pub const PICKUP_RADIUS: f32 = 18.0;

pub struct TreasurePlugin;

impl Plugin for TreasurePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Treasury>()
            .add_systems(Update, (collect_coins, spin_coins));
    }
}

pub fn collect_coins(
    mut commands: Commands,
    mut treasury: ResMut<Treasury>,
    player: Single<&Transform, With<Player>>,
    coins: Query<(Entity, &Transform), With<Coin>>,
) {
    let p = player.translation.truncate();
    for (entity, transform) in &coins {
        if transform.translation.truncate().distance(p) <= PICKUP_RADIUS {
            commands.entity(entity).despawn();
            treasury.gold += 10;
        }
    }
}

fn spin_coins(time: Res<Time>, mut coins: Query<&mut Transform, With<Coin>>) {
    for mut t in &mut coins {
        t.rotate_z(2.0 * time.delta_secs());
    }
}
