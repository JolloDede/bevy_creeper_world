use bevy::{
    color::palettes::tailwind::{GREEN_400, RED_800},
    prelude::*,
    time::Stopwatch,
};
use strum::{Display, EnumIter, EnumString};

use crate::{
    creeper_world::GameState,
    game::resource::{CurrentEnergy, MaxEnergy},
};

mod hud;
use hud::*;
mod resource;

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
        app.add_systems(
            Update,
            (mission_time_system, status_lines_system).run_if(in_state(self._state.clone())),
        );
        // app.add_systems(OnExit(GameState::Start), cleanup_menu);

        app.insert_resource(MissionTimer(Stopwatch::new()));
        app.insert_resource(CurrentEnergy(60));
        app.insert_resource(MaxEnergy(60));
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
        let green = Color::srgb_u8(0, 128, 0);
        let red = Color::srgb_u8(128, 0, 0);
        match b_type {
            BuildingType::Collector => Color::from(green),
            BuildingType::Relay => Color::from(green),
            BuildingType::Storage => Color::from(green),
            BuildingType::Speed => Color::from(green),
            BuildingType::Reactor => Color::from(green),
            BuildingType::Blaster => Color::from(red),
            BuildingType::Mortar => Color::from(red),
            BuildingType::SAM => Color::from(red),
            BuildingType::Drone => Color::from(red),
        }
    }
}

#[derive(Resource)]
pub struct MissionTimer(Stopwatch);

#[derive(Component)]
pub struct MissionTimerMarker;

fn mission_time_system(
    mission_time_query: Single<&mut Text, With<MissionTimerMarker>>,
    time: Res<Time>,
    mut mission_time: ResMut<MissionTimer>,
) {
    let mut mt_text = mission_time_query.into_inner();

    mission_time.0.tick(time.delta());

    mt_text.0 = format!(
        "{}:{:02}",
        ((mission_time.0.elapsed_secs() / 60.) % 60.).floor(),
        (mission_time.0.elapsed_secs() % 60.).floor()
    );
}
