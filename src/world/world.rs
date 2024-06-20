use crate::actor::ActorPlugins;
use crate::component::ComponentPlugins;
use crate::lobby::{LobbyPlugins, LobbyState};
use crate::map::MapPlugins;
use crate::settings::SettingsPlugins;
use crate::sound::SoundPlugins;
use crate::ui::UiPlugins;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::SpawnPoint;

/// Enum representing collision layers for physics interactions.
// TODO: #[derive(PhysicsLayer)]
pub enum CollisionLayer {
    /// Actors with this layer cannot collide with each other.
    ActorNoclip,
    /// The default collision layer.
    Default,
}

/// A component representing a promised GLTF scene.
///
/// This component is used to temporarily hold a GLTF scene while additional components are added to it.
/// Once processing is complete, it should be removed from the entity.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use pih_pah_app::world::PromisedScene;
///
/// fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
///     let scene = asset_server.load("my_scene.glb#Scene0");
///
///     // Create an entity with the PromisedScene component.
///     commands.spawn((
///         SceneBundle { scene, ..default() },
///         PromisedScene,
///     ));
/// }
///```
#[derive(Component)]
pub struct PromisedScene;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LinkId {
    Scene(String),
    Projectile(usize),
}

#[derive(Resource, Default, Reflect, Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut)]
pub struct ProjectileIdSeq(usize);

impl ProjectileIdSeq {
    /// Returns the next projectile ID. A new ID is generated each time this method is called.
    pub fn shift(&mut self) -> LinkId {
        self.0 += 1;
        LinkId::Projectile(self.0)
    }
}

pub struct WorldPlugins;

impl Plugin for WorldPlugins {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectileIdSeq>()
            .register_type::<ProjectileIdSeq>()
            .add_plugins((
                SettingsPlugins,
                SoundPlugins,
                MapPlugins,
                UiPlugins,
                LobbyPlugins,
                ActorPlugins,
                ComponentPlugins,
      ));
    }
}

#[derive(Component)]
pub struct Me;

fn init() {

}
