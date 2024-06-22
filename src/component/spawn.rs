use bevy::{
    app::{App, Plugin, Update},
    ecs::{component::Component, query::With, system::{Query, ResMut, Commands}, entity::Entity},
    reflect::Reflect,
    transform::components::GlobalTransform,
};

use crate::world;

#[derive(Component, Reflect, Default, Debug)]
pub struct SpawnPoint;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(world::SpawnProperty::empty())
      // TODO: process_spawn_point in state?
            .register_type::<SpawnPoint>().add_systems(Update, process_spawn_point);
    }
}

fn process_spawn_point(
  mut commands: Commands,
  query: Query<(Entity, &GlobalTransform), With<SpawnPoint>>,
  mut resource: ResMut<world::SpawnProperty>,
) {
    // TODO: spawn point not only like vec3 but like entity (moveble point)
    for (entity, global_transform) in &query {
         resource.push(global_transform.translation());
         commands.entity(entity).despawn(); // TODO: ugly realization
    }
}
