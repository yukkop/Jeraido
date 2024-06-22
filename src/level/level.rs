use bevy::prelude::*;

use crate::{world::SpawnProperty, lobby::LevelCode};

use super::{hub::HubPlugins, custom::CustomPlugins};

#[derive(Component)]
pub struct Affiliation(pub LevelCode);

pub struct MapPlugins;

impl Plugin for MapPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnProperty>().add_plugins((HubPlugins, CustomPlugins));
    }
}
