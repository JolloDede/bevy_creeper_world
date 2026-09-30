use bevy::prelude::*;

use crate::creeper_world::ButtonLightMaterial;

pub fn button_comp(
    text: String,
    color: Color,
    marker: impl Component,
    group: impl Component,
    ui_material: &mut ResMut<Assets<ButtonLightMaterial>>,
) -> impl Bundle {
    (
        Node {
            padding: UiRect::axes(px(8), px(2)),
            border: UiRect::all(px(2)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(color),
        marker,
        group,
        MaterialNode(ui_material.add(ButtonLightMaterial::new(color))),
        BoxShadow(vec![ShadowStyle {
            color: Color::BLACK.with_alpha(0.8),
            x_offset: px(20),
            y_offset: px(20),
            spread_radius: px(15),
            blur_radius: px(19),
        }]),
        Button,
        children![Text::new(text),],
    )
}
