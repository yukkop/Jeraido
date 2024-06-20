use bevy::prelude::*;
use bevy_controls::{resource::PlayerActions, contract::InputsContainer};
use bevy_controls_derive::{Action, GameState};
use strum_macros::EnumIter;

use crate::{controls::ControlsPlugins, world::WorldPlugins};

#[derive(PartialEq, Eq, Hash, EnumIter, Clone, Copy, Debug, Action)]
pub enum CoreAction {
    InGameMenu,
}

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default, GameState)]
pub enum CoreGameState {
    InGame,
    #[default]
    Hub,
}

/// Main plugin of the game
pub struct CorePlugins;

impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
          WorldPlugins,
          ControlsPlugins,
        ));
    }
}
