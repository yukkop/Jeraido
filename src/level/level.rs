use bevy::prelude::*;

use crate::{world::SpawnPoint, lobby::LevelCode};

use super::{hub::HubPlugins, custom::CustomPlugins};

#[derive(Component)]
pub struct Affiliation(pub LevelCode);

pub fn is_loaded(spawn_point: &Res<SpawnPoint>) -> bool {
    !spawn_point.is_empty()
}

pub struct MapPlugins;

impl Plugin for MapPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnPoint>().add_plugins((HubPlugins, CustomPlugins));
    }
}
