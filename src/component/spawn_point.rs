use bevy::ecs::component::Component;

#[derive(Component, Debug)]
pub struct SpawnPont;

pub struct ComponentPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpawnPoint>();
    }
}
