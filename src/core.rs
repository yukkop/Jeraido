
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

#[derive(Resource, Default, Clone, Debug)]
pub struct Lobby {
    // When the game does not provide multiplayer, one field is enough
    player_inputs: PlayerActions<CoreAction>,
}

impl InputsContainer<CoreAction> for Lobby {
    fn iter_inputs<'a>(&'a self) -> Box<dyn Iterator<Item = &'a PlayerActions<CoreAction>> + 'a> {
        todo!()
    }

    fn me<'a>(&'a self) -> Option<&'a PlayerActions<CoreAction>> {
        Some(&self.player_inputs)
    }

    fn me_mut<'a>(&'a mut self) -> Option<&'a mut PlayerActions<CoreAction>> {
        Some(&mut self.player_inputs)
    }
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
