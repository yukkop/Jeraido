use bevy::{
    app::{App, Plugin},
    asset::{Assets},
    core::Name,
    ecs::{
        component::Component,
        reflect::ReflectComponent,
        schedule::OnEnter,
        system::{Commands, Query, Res},
    },
    reflect::Reflect,
    scene::SceneBundle,
    utils::default,
};
use bevy_gltf_components::ComponentsFromGltfPlugin;


use crate::{
    component::ComponentsTestPlugin,
    core::{CoreGameState, GameLevel}, world::SpawnProperty,
};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct LoadedMarker;

pub struct CustomPlugins;

impl Plugin for CustomPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(ComponentsFromGltfPlugin::default(),)
            .add_systems(OnEnter(CoreGameState::InGame), spawn_level);
    }
}

fn spawn_level(
    mut commands: Commands,
    scene_markers: Query<&LoadedMarker>,
    model_assets: Res<GameLevel>,
    models: Res<Assets<bevy::gltf::Gltf>>,
) {
    commands.insert_resource(SpawnProperty::empty());
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
