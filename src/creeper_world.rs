use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::{game::GamePlugin, start_screen::StartScreenPlugin};

pub struct CreeperWorldPlugins;

impl PluginGroup for CreeperWorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(StartScreenPlugin::new(GameState::Start))
            .add(GamePlugin::new(GameState::InGame))
        // .add(HudPlugin::new(GameState::InGame))
        // .add(EndOfGamePlugin::new(GameState::GameOver))
    }
}

#[derive(States, Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Start,
    InGame,
    GameOver,
}
