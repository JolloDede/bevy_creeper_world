use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

mod creeper_world;
use creeper_world::*;

mod components;
mod game;
mod start_screen;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Creeper World".to_string(),
                        resolution: (800, 400).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(LogPlugin {
                    level: Level::INFO,
                    ..Default::default()
                }),
        )
        .add_plugins(CreeperWorldPlugins)
        .run();
}
