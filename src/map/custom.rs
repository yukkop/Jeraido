use bevy::{app::{Plugin, App}, ecs::schedule::OnEnter};
use hmac::digest::Update;

use crate::core::CoreGameState;

pub struct HubPlugins;

impl Plugin for HubPlugins {
    fn build(&self, app: &mut App) {
        app;
    }
}
