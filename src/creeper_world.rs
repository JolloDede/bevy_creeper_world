use bevy::{
    app::PluginGroupBuilder, prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef,
};

use crate::{game::GamePlugin, start_screen::StartScreenPlugin};

pub struct CreeperWorldPlugins;

impl PluginGroup for CreeperWorldPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(StartScreenPlugin::new(GameState::Start))
            .add(GamePlugin::new(GameState::InGame))
            // .add(HudPlugin::new(GameState::InGame))
            // .add(EndOfGamePlugin::new(GameState::GameOver))
            .add(UiMaterialPlugin::<ButtonLightMaterial>::default())
    }
}

#[derive(States, Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Start,
    SelectLevel,
    InGame,
    GameOver,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ButtonLightMaterial {
    #[uniform(0)]
    base_color: Vec4,
    #[uniform(1)]
    light_pos: Vec4,
}

impl ButtonLightMaterial {
    pub fn new(color: Color) -> Self {
        ButtonLightMaterial {
            base_color: color.to_linear().to_vec4(),
            light_pos: Vec4::new(0.5, 1., 0., 0.),
        }
    }
}

impl UiMaterial for ButtonLightMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/button_light.wgsl".into()
    }
}
