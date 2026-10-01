use bevy::{prelude::*, ui::Display::Grid};
use strum::IntoEnumIterator;

use crate::{
    components::button_comp,
    consts::HUD_MENU_HEIGTH_PERCENT,
    creeper_world::ButtonLightMaterial,
    game::{BuildingComp, BuildingType, MissionTimerMarker},
};

pub fn setup_hud(mut commands: Commands, mut ui_material: ResMut<Assets<ButtonLightMaterial>>) {
    let parent = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                margin: UiRect::top(Val::Auto),
                bottom: Val::ZERO,
                height: percent(HUD_MENU_HEIGTH_PERCENT),
                width: percent(100),
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .id();

    let turret_button_parent = commands
        .spawn((
            Node {
                display: Grid,
                grid_template_columns: vec![
                    GridTrack::flex(1.),
                    GridTrack::flex(1.),
                    GridTrack::flex(1.),
                    GridTrack::flex(1.),
                    GridTrack::flex(1.),
                ],
                ..default()
            },
            BackgroundColor(Color::srgb_u8(128, 128, 128)),
        ))
        .id();
    let menu_button_parent = commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .id();

    let mission_time_parent = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::axes(px(8), px(4)),
                border: UiRect::all(px(1)),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(0, 0, 120)),
            BorderColor::all(Color::srgb_u8(240, 240, 240)),
        ))
        .id();
    let mission_time_container = commands
        .spawn(
            (Node {
                flex_direction: FlexDirection::Column,
                ..default()
            }),
        )
        .id();
    commands.entity(parent).add_child(turret_button_parent);
    commands.entity(parent).add_child(mission_time_container);
    commands
        .entity(mission_time_container)
        .add_child(mission_time_parent);
    commands.entity(parent).add_child(menu_button_parent);

    create_turret_buttons(&mut commands, turret_button_parent, &mut ui_material);
    // spacer
    // upgrades
    // elevation
    // mission time
    create_misson_time(&mut commands, mission_time_parent);
    // status line
    // sound buttons
    create_menu_buttons(&mut commands, menu_button_parent, &mut ui_material);
}

#[derive(Component)]
struct HudButtons;

fn create_turret_buttons(
    commands: &mut Commands,
    parent: Entity,
    ui_material: &mut ResMut<Assets<ButtonLightMaterial>>,
) {
    let mut children = Vec::new();

    for b_type in BuildingType::iter() {
        let bundle = button_comp(
            b_type.to_string(),
            BuildingType::get_button_color(&b_type),
            BuildingComp(b_type),
            HudButtons,
            ui_material,
        );
        children.push(commands.spawn(bundle).id());
    }

    commands.entity(parent).add_children(&children);
}

#[derive(Component)]
struct OptionsButton;

#[derive(Component)]
struct HelpButton;

#[derive(Component)]
struct ExitButton;

fn create_menu_buttons(
    commands: &mut Commands,
    parent: Entity,
    ui_material: &mut ResMut<Assets<ButtonLightMaterial>>,
) {
    commands.entity(parent).with_children(|parent| {
        parent.spawn(button_comp(
            "Options".to_string(),
            Color::srgb_u8(128, 128, 0),
            OptionsButton,
            HudButtons,
            ui_material,
        ));
        parent.spawn(button_comp(
            "Help".to_string(),
            Color::srgb_u8(0, 240, 0),
            HelpButton,
            HudButtons,
            ui_material,
        ));
        parent.spawn(button_comp(
            "Exit Game".to_string(),
            Color::srgb_u8(80, 0, 0),
            ExitButton,
            HudButtons,
            ui_material,
        ));
    });
}

fn create_misson_time(commands: &mut Commands, parent: Entity) {
    let mt_text = commands
        .spawn((Text::new("Mission Time"), TextColor(Color::WHITE)))
        .id();

    let mtm_parent = commands
        .spawn((Node {
            margin: UiRect::horizontal(Val::Auto),
            ..default()
        },))
        .id();
    let mtm_text = commands
        .spawn((
            Text::new("0:00"),
            TextColor(Color::srgb_u8(240, 0, 0)),
            MissionTimerMarker,
        ))
        .id();

    commands.entity(parent).add_child(mt_text);
    commands.entity(parent).add_child(mtm_parent);
    commands.entity(mtm_parent).add_child(mtm_text);
}
