use std::fmt::Display;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world::SpawnPoint;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Serialize, Deserialize)]
pub enum MapState {
    #[default]
    Menu = 0,
    Arena,
}

impl Display for MapState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapState::Menu => write!(f, "Menu"),
            MapState::Arena => write!(f, "Arena"),
        }
    }
}

pub fn is_loaded(spawn_point: &Res<SpawnPoint>) -> bool {
    !spawn_point.is_empty()
}

pub struct MapPlugins;

impl Plugin for MapPlugins {
    fn build(&self, app: &mut App) {
        app
            .insert_state(MapState::default())
            .init_resource::<SpawnPoint>()
            .add_plugins(());
    }
}
