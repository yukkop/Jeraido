use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;

use bevy_controls_derive::{Action, GameState};
use bevy_kira_audio::AudioSource;
use strum_macros::EnumIter;

use crate::{controls::ControlsPlugins, lobby::LevelCode, world::WorldPlugins};

#[derive(PartialEq, Eq, Hash, EnumIter, Clone, Copy, Debug, Action)]
pub enum CoreAction {
    InGameMenu,
}

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default, GameState)]
pub enum CoreGameState {
    #[default]
    PrimaryLoad,
    Hub,
    LoadCustomLevel,
    InGame,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum KnownLevel {
    Hub,
}

#[derive(Debug, Event, Deref, DerefMut)]
pub struct LoadLevelEvent(pub LevelCode);

#[derive(AssetCollection, Resource)]
pub struct GameLevel {
    #[asset(key = "level")]
    pub level: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(key = "sounds.background")]
    pub background: Handle<AudioSource>,
}

/// Main plugin of the game
pub struct CorePlugins;

impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadLevelEvent>()
            .add_loading_state(
                LoadingState::new(CoreGameState::PrimaryLoad)
                    .continue_to_state(CoreGameState::Hub)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                        "primary.assets.ron",
                    )
                    .load_collection::<AudioAssets>(),
            )
            .add_loading_state(
                LoadingState::new(CoreGameState::LoadCustomLevel)
                    .continue_to_state(CoreGameState::InGame)
                    .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                        "dynamic_map.assets.ron",
                    )
                    .load_collection::<GameLevel>(),
            )
            .add_plugins((WorldPlugins, ControlsPlugins))
            .add_systems(Update, load_level_event);
    }
}

fn load_level_event(
    mut load_level_event: EventReader<LoadLevelEvent>,
    mut next_state: ResMut<NextState<CoreGameState>>,
) {
    if let Some(_event) = load_level_event.read().next() {
        next_state.set(CoreGameState::LoadCustomLevel);
    }
}
