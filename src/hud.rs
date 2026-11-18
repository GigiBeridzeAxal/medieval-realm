//! On-screen text: gold counter and controls help.

use crate::components::*;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud).add_systems(
            Update,
            update_gold_text.run_if(resource_changed::<Treasury>),
        );
    }
}

#[derive(Component)]
struct GoldText;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        GoldText,
        Text::new("Gold: 0"),
        TextColor(Color::srgb(1.0, 0.85, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
    commands.spawn((
        Text::new("WASD / Arrows: move   Collect all the gold!"),
        TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));
}

pub fn gold_label(t: &Treasury) -> String {
    let collected = t.gold / 10;
    if t.total_coins > 0 && collected >= t.total_coins {
        format!(
            "Gold: {}  -  All {} coins found! The realm is rich!",
            t.gold, t.total_coins
        )
    } else {
        format!("Gold: {}  ({}/{} coins)", t.gold, collected, t.total_coins)
    }
}

fn update_gold_text(treasury: Res<Treasury>, mut text: Single<&mut Text, With<GoldText>>) {
    text.0 = gold_label(&treasury);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels() {
        assert_eq!(
            gold_label(&Treasury {
                gold: 20,
                total_coins: 5
            }),
            "Gold: 20  (2/5 coins)"
        );
        assert!(
            gold_label(&Treasury {
                gold: 50,
                total_coins: 5
            })
            .contains("realm is rich")
        );
    }
}
