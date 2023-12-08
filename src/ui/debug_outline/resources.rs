use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct ShowOutlines(pub bool);
