use bevy::prelude::*;

#[derive(Resource)]
pub struct MaxEnergy(pub u8);

#[derive(Resource)]
pub struct CurrentEnergy(pub u8);
