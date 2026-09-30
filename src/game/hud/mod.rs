use bevy::{prelude::*, ui::Display::Grid};
use strum::IntoEnumIterator;

use crate::{
    components::button_comp,
    consts::HUD_MENU_HEIGTH_PERCENT,
    creeper_world::ButtonLightMaterial,
    game::{BuildingComp, BuildingType},
};

pub fn setup_hud(mut commands: Commands, ui_material: ResMut<Assets<ButtonLightMaterial>>) {
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

    commands.entity(parent).add_child(turret_button_parent);
    create_turret_buttons(&mut commands, turret_button_parent, ui_material);
}

#[derive(Component)]
struct TurretButtons;

fn create_turret_buttons(
    commands: &mut Commands,
    parent: Entity,
    mut ui_material: ResMut<Assets<ButtonLightMaterial>>,
) {
    let mut children = Vec::new();

    for b_type in BuildingType::iter() {
        let bundle = button_comp(
            b_type.to_string(),
            BuildingType::get_button_color(&b_type),
            BuildingComp(b_type),
            TurretButtons,
            &mut ui_material,
        );
        children.push(commands.spawn(bundle).id());
    }

    commands.entity(parent).add_children(&children);
}
