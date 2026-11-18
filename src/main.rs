use bevy::prelude::*;
use medieval_realm::MedievalGamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Medieval Realm".into(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                })
                // Crisp pixels for pixel art.
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(MedievalGamePlugin)
        .run();
}
