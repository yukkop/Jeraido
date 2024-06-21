use bevy::prelude::*;
use bevy_controls::{resource::PlayerActions, contract::InputsContainer};
use bevy_controls_derive::{Action, GameState};
use bevy_kira_audio::AudioSource;
use strum_macros::EnumIter;
use bevy_asset_loader::prelude::*;

use crate::{controls::ControlsPlugins, world::WorldPlugins};

#[derive(PartialEq, Eq, Hash, EnumIter, Clone, Copy, Debug, Action)]
pub enum CoreAction {
    InGameMenu,
}

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default, GameState)]
pub enum CoreGameState {
    #[default]
    PrimaryLoad,
    Hub,
    LoadCustomMap,
    InGame,
}

/// Main plugin of the game
pub struct CorePlugins;

impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app
        .add_loading_state(
            LoadingState::new(CoreGameState::PrimaryLoad)
                .continue_to_state(CoreGameState::Hub)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "primary.assets.ron",
                )
                .load_collection::<AudioAssets>(),
        )
        .add_plugins((
          WorldPlugins,
          ControlsPlugins,
        ));
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(key = "sounds.background")]
    pub background: Handle<AudioSource>,
}
