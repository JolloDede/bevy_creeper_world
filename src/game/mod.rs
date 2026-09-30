use bevy::prelude::*;
use strum::{Display, EnumIter, EnumString};

use crate::creeper_world::GameState;

mod hud;
use hud::*;

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
        app.add_systems(OnEnter(GameState::InGame), setup_hud);
        // app.add_systems(Update, button_system);
        // app.add_systems(OnExit(GameState::Start), cleanup_menu);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default()));
}

#[derive(Component)]
pub struct BuildingComp(pub BuildingType);

#[derive(EnumIter, Display, Debug, PartialEq, Clone, Copy)]
pub enum BuildingType {
    Collector,
    Relay,
    Storage,
    Speed,
    Reactor,
    Blaster,
    Mortar,
    SAM,
    Drone,
}

impl BuildingType {
    pub fn get_button_color(b_type: &BuildingType) -> Color {
        match b_type {
            BuildingType::Collector => Color::srgb_u8(0, 240, 0),
            BuildingType::Relay => Color::srgb_u8(0, 240, 0),
            BuildingType::Storage => Color::srgb_u8(0, 240, 0),
            BuildingType::Speed => Color::srgb_u8(0, 240, 0),
            BuildingType::Reactor => Color::srgb_u8(0, 240, 0),
            BuildingType::Blaster => Color::srgb_u8(240, 0, 0),
            BuildingType::Mortar => Color::srgb_u8(240, 0, 0),
            BuildingType::SAM => Color::srgb_u8(240, 0, 0),
            BuildingType::Drone => Color::srgb_u8(240, 0, 0),
        }
    }
}
