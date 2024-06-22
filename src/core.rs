use std::{path::Path, fs::OpenOptions, io::Write};

use bevy::{gltf::Gltf, prelude::*};
use bevy_asset_loader::prelude::*;

use bevy_controls_derive::{Action, GameState};
use bevy_kira_audio::AudioSource;
use strum_macros::EnumIter;

use crate::{controls::ControlsPlugins, lobby::LevelCode, world::WorldPlugins, ASSET_DIR};

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

#[derive(Debug, Event, Deref, DerefMut, Clone)]
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

        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            change_state_log.run_if(state_changed::<CoreGameState>),
        );
    }
}

#[cfg(debug_assertions)]
fn change_state_log(core_state: Res<State<CoreGameState>>) {
    log::debug!("new state: {:#?}", core_state);
}

fn load_level_event(
    mut load_level_event: EventReader<LoadLevelEvent>,
    mut next_state: ResMut<NextState<CoreGameState>>,
) { 
    if let Some(event) = load_level_event.read().next() {
        match &**event {
            LevelCode::Path(path) => {
                log::info!("load level: {}", path);
                let path = Path::new(ASSET_DIR).join("level").join(format!("{path}.glb"));
                let path_ron = Path::new(ASSET_DIR).join("dynamic_map.assets.ron");

                if path.exists() {
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(path_ron).unwrap();

                    file.write_all(br#"({
                       "level": File (
                          path: "level/Level1.glb",
                        ),
                    })
                    "#).unwrap();
                    next_state.set(CoreGameState::LoadCustomLevel);
                } else {
                    log::error!("{:#?} not exist in map folder", path);
                }
            }
            LevelCode::Url(_url) => todo!(),
            LevelCode::Known(known_level) => {
                log::info!("load level: {:#?}", known_level);
                match known_level {
                    KnownLevel::Hub => next_state.set(CoreGameState::Hub),
                }
            }
        }
    }
}
