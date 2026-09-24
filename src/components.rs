use bevy::prelude::*;

pub fn button_comp(text: String, color: Color, marker: impl Component) -> impl Bundle {
    (
        Node {
            padding: UiRect::axes(px(8), px(2)),
            border: UiRect::all(px(2)),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(color),
        marker,
        children![Text::new(text),],
    )
}
