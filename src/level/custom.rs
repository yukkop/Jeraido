use bevy::{
    app::{App, Plugin},
    asset::{AssetEvent, Assets},
    core::Name,
    ecs::{
        component::Component,
        event::EventReader,
        reflect::ReflectComponent,
        schedule::OnEnter,
        system::{Commands, Query, Res, ResMut},
    },
    gltf::Gltf,
    reflect::Reflect,
    scene::SceneBundle,
    utils::default,
};
use bevy_gltf_components::ComponentsFromGltfPlugin;
use hmac::digest::Update;

use crate::{
    component::ComponentsTestPlugin,
    core::{CoreGameState, GameLevel},
};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct LoadedMarker;

pub struct HubPlugins;

impl Plugin for HubPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((ComponentsFromGltfPlugin::default(), ComponentsTestPlugin))
            .add_systems(OnEnter(CoreGameState::InGame), spawn_level);
    }
}

fn spawn_level(
    mut commands: Commands,
    scene_markers: Query<&LoadedMarker>,
    model_assets: Res<GameLevel>,
    models: Res<Assets<bevy::gltf::Gltf>>,
) {
    let gltf = models.get(model_assets.level.clone()).unwrap();
    if scene_markers.is_empty() {
        log::info!("spawning scene");
        commands.spawn((
            SceneBundle {
                scene: gltf.scenes[0].clone(),
                ..default()
            },
            LoadedMarker,
            Name::new("Level1"),
        ));
    } else {
        log::error!("scene already exist");
    }
}
