use std::fmt::Formatter;

use bevy::{ecs::system::SystemId, prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::{controls::{ControlsPlugins, CoreAction}, world::WorldPlugins};

/// Main plugin of the game
pub struct CorePlugins;

impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
          ControlsPlugins,
          WorldPlugins,
        ));
    }
}
