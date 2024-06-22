use crate::{core::CoreGameState, ui::MainCamera};

use bevy::prelude::*;
use std::f32::consts::PI;

use super::Affiliation;

const PRIMARY_CAMERA_ORDER: isize = 3;

const HUB_CODE: &str = "this is hub epta";

#[derive(Component)]
struct OrbitLight {
    radius: f32,
    speed: f32,
    angle: f32,
}

pub struct HubPlugins;

impl Plugin for HubPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CoreGameState::Hub), load)
            .add_systems(
                Update,
                update_light_position.run_if(in_state(CoreGameState::Hub)),
            )
            .add_systems(OnExit(CoreGameState::Hub), unload);
    }
}

fn load(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(5., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
                camera: Camera {
                    order: PRIMARY_CAMERA_ORDER,
                    ..default()
                },
                ..Default::default()
            },
            MainCamera,
        ))
        .insert(Affiliation(HUB_CODE.into()));

    commands
        .spawn((
            PointLightBundle {
                point_light: PointLight {
                    intensity: 5000.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_xyz(0., 8.0, 0.),
                ..default()
            },
            OrbitLight {
                radius: 8.0,
                speed: 1.0,
                angle: 0.0,
            },
        ))
        .insert(Affiliation(HUB_CODE.into()));

    commands
        .spawn((
            PbrBundle {
                mesh: mesh.add(Mesh::from(Plane3d::new(Vec3::Y))),
                material: materials.add(Color::GREEN),
                transform: Transform::from_xyz(0., 0., 0.),
                ..Default::default()
            },
            Name::new("Terrain"),
        ))
        .insert(Affiliation(HUB_CODE.into()));

    commands
        .spawn((
            PbrBundle {
                mesh: mesh
      .add(Mesh::from(Cuboid::from_size(Vec3::new(0.5, 0.5, 0.5)))),
                material: materials.add(Color::GRAY),
                transform: Transform::from_xyz(0., 0.5, 0.),
                ..Default::default()
            },
            Name::new("Cube"),
        ))
        .insert(Affiliation(HUB_CODE.into()));
}

fn unload(mut commands: Commands, affiliation_query: Query<Entity, With<Affiliation>>) {
    for entity in affiliation_query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_light_position(time: Res<Time>, mut query: Query<(&mut OrbitLight, &mut Transform)>) {
    for (mut orbit_light, mut transform) in query.iter_mut() {
        orbit_light.angle += orbit_light.speed * time.delta_seconds();
        if orbit_light.angle > 2.0 * PI {
            orbit_light.angle -= 2.0 * PI;
        }
        transform.translation.x = orbit_light.radius * orbit_light.angle.cos();
        transform.translation.z = orbit_light.radius * orbit_light.angle.sin();
    }
}
