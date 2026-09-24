use bevy::prelude::*;

use crate::creeper_world::GameState;

pub struct GamePlugin<S: States> {
    _state: S,
}

impl<S: States> GamePlugin<S> {
    pub fn new(s: S) -> Self {
        Self { _state: s }
    }
}

impl<S: States> Plugin for GamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        // app.add_systems(OnEnter(GameState::Start), setup);
        // app.add_systems(Update, button_system);
        // app.add_systems(OnExit(GameState::Start), cleanup_menu);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default()));
}
