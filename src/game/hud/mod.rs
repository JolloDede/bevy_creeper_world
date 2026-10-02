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

    let turret_buttons = create_turret_buttons(&mut commands, &mut ui_material);
    // spacer
    // upgrades
    // elevation
    // mission time
    let mission_time = create_misson_time(&mut commands);
    // status line
    let status_lines = create_status_lines(&mut commands);
    // sound buttons
    let menu_buttons = create_menu_buttons(&mut commands, &mut ui_material);
    commands.entity(parent).add_children(&[
        turret_buttons,
        mission_time,
        status_lines,
        menu_buttons,
    ]);
}

#[derive(Component)]
struct HudButtons;

fn create_turret_buttons(
    commands: &mut Commands,
    ui_material: &mut ResMut<Assets<ButtonLightMaterial>>,
) -> Entity {
    let parent = commands
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

    return parent;
}

#[derive(Component)]
struct OptionsButton;

#[derive(Component)]
struct HelpButton;

#[derive(Component)]
struct ExitButton;

fn create_menu_buttons(
    commands: &mut Commands,
    ui_material: &mut ResMut<Assets<ButtonLightMaterial>>,
) -> Entity {
    let parent = commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },))
        .id();

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

    return parent;
}

fn create_misson_time(commands: &mut Commands) -> Entity {
    let parent = commands
        .spawn(
            (Node {
                flex_direction: FlexDirection::Column,
                ..default()
            }),
        )
        .id();

    let mission_time_container = commands
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

    commands.entity(parent).add_child(mission_time_container);
    commands
        .entity(mission_time_container)
        .add_children(&[mt_text, mtm_parent]);
    commands.entity(mtm_parent).add_child(mtm_text);

    return parent;
}

fn create_status_lines(commands: &mut Commands) -> Entity {
    let parent = commands
        .spawn(
            (Node {
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.), GridTrack::flex(1.)],
                grid_template_rows: vec![
                    GridTrack::flex(0.8),
                    GridTrack::flex(0.4),
                    GridTrack::flex(0.4),
                    GridTrack::flex(0.4),
                ],
                row_gap: px(4),
                column_gap: px(4),
                margin: UiRect::axes(px(4), px(0)),
                ..default()
            }),
        )
        .id();

    commands.entity(parent).with_children(|parent| {
        parent.spawn(Text::new("Energy"));
        parent.spawn(status_bg_bundle(
            16.,
            EnergyTextMarkerComponent,
            Color::hsl(120., 1., 0.4),
            EnergyPgMarkerComponent,
        ));
        parent.spawn(status_text("Collection"));
        parent.spawn(status_bg_bundle(
            14.,
            CollectionTextMarkerComponent,
            Color::hsl(120., 1., 0.5),
            CollectionPgMarkerComponent,
        ));
        parent.spawn(status_text("Depletion"));
        parent.spawn(status_bg_bundle(
            14.,
            DepletionTextMarkerComponent,
            Color::hsl(60., 1., 0.5),
            DepletionPgMarkerComponent,
        ));
        parent.spawn(status_text("Saturation"));
        parent.spawn(status_bg_bundle(
            14.,
            SaturationTextMarkerComponent,
            Color::hsl(0., 1., 0.5),
            SaturationPgMarkerComponent,
        ));
    });

    return parent;
}

fn status_bg_bundle(
    font_size: f32,
    text_marker: impl Component,
    pb_color: Color,
    pb_marker: impl Component,
) -> impl Bundle {
    (
        Node { ..default() },
        BackgroundColor(Color::srgb_u8(128, 128, 128)),
        children![
            (
                Node {
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
                children![(
                    Text::new("0/60"),
                    TextFont {
                        font_size: font_size,
                        ..default()
                    },
                    text_marker,
                )],
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    width: percent(25),
                    left: px(0),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                BackgroundColor(pb_color),
                pb_marker,
            )
        ],
    )
}

fn status_text(text: &str) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 14.,
            ..default()
        },
    )
}

#[derive(Component)]
struct EnergyTextMarkerComponent;
#[derive(Component)]
struct EnergyPgMarkerComponent;

#[derive(Component)]
struct CollectionTextMarkerComponent;
#[derive(Component)]
struct CollectionPgMarkerComponent;

#[derive(Component)]
struct DepletionTextMarkerComponent;
#[derive(Component)]
struct DepletionPgMarkerComponent;

#[derive(Component)]
struct SaturationTextMarkerComponent;
#[derive(Component)]
struct SaturationPgMarkerComponent;
