use bevy::prelude::*;

use crate::ButtonLightMaterial;
use crate::components::button_comp;
use crate::creeper_world::GameState;

pub struct StartScreenPlugin<S: States> {
    _state: S,
}

impl<S: States> StartScreenPlugin<S> {
    pub fn new(s: S) -> Self {
        Self { _state: s }
    }
}

impl<S: States> Plugin for StartScreenPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), setup);
        app.add_systems(Update, button_system);
        app.add_systems(OnExit(GameState::Start), cleanup_menu);

        app.init_state::<GameState>();
    }
}

#[derive(Component)]
struct StartScreenMarker;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ui_material: ResMut<Assets<ButtonLightMaterial>>,
) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        ImageNode::from(asset_server.load("background/start_screen.png")),
        StartScreenMarker,
    ));

    let mission_button = button_comp(
        "Missions".to_string(),
        Color::srgb_u8(255, 0, 0),
        MissionButton,
        &mut ui_material,
    );
    let how_to_button = button_comp(
        "How to play".to_string(),
        Color::srgb_u8(0, 255, 0),
        HowToButton,
        &mut ui_material,
    );

    commands.spawn((
        Node {
            margin: UiRect::horizontal(Val::Auto),
            top: percent(10),
            width: percent(20),
            flex_direction: FlexDirection::Column,
            row_gap: px(8),
            ..default()
        },
        StartScreenMarker,
        ZIndex(1),
        children![mission_button, how_to_button],
    ));
}

#[derive(Component)]
pub struct StartMenuButtons;

fn button_system(
    action_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<StartMenuButtons>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mission_query: Query<&MissionButton>,
) {
    for (interaction, entity) in action_query.iter() {
        match interaction {
            Interaction::Pressed => {
                if let Ok(_) = mission_query.get(entity) {
                    game_state.set(GameState::InGame);
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn cleanup_menu(mut commands: Commands, screen_query: Query<Entity, With<StartScreenMarker>>) {
    for entity in screen_query {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct MissionButton;

#[derive(Component)]
pub struct HowToButton;
