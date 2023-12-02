use bevy::prelude::*;

#[derive(Resource, Deref)]
pub struct ShowFps(pub bool);
