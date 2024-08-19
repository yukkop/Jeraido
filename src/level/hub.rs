use crate::{core::{CoreGameState, KnownLevel}, ui::MainCamera, lobby::LevelCode};
use voronoi::{voronoi, Point, make_polygons};

use bevy::{prelude::*, render::{mesh::{Indices, PrimitiveTopology}, render_asset::RenderAssetUsages}};
use std::f32::consts::PI;

use super::Affiliation;

const PRIMARY_CAMERA_ORDER: isize = 3;

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

fn make_mesh(polygon: Vec<Point>) -> Mesh {
    // Given data: Vec<Point>
    let points: Vec<Vec3> = polygon.into_iter().map(|e| {
        Vec3 {x: e.x.0 as f32, y: 0., z: e.y.0 as f32}
    }).collect();

    // Ensure you have at least 3 points to form a plane
    if points.len() < 3 {
        panic!("Expected at least 3 points to form a plane");
    }

    // Calculate indices for a triangulated surface (assuming a convex polygon)
    let mut indices = Vec::new();
    for i in 1..points.len() - 1 {
        indices.push(0 as u32);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }

    // Generate a normal (assuming points lie on a plane)
    let normal = (points[1] - points[0]).cross(points[2] - points[0]).normalize();
    let normals = vec![normal; points.len()];

    // Calculate UV coordinates (using a basic planar mapping technique)
    let uvs: Vec<Vec2> = points
        .iter()
        .map(|p| Vec2::new(p.x, p.z)) // Assuming x and z are the primary plane axes
        .collect();

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn load(
    mut commands: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const BOX_SIZE: f64 = 800.;
    let vor_pts = vec![
        Point::new(0.0, 1.0),
        Point::new(2.0, 3.0),
        Point::new(10.0, 12.0)
    ];
    let vor_diagram = voronoi(vor_pts, BOX_SIZE);
    let vor_polys = make_polygons(&vor_diagram);
     
    for (index, vor_poly) in vor_polys.iter().enumerate() {
      commands
        .spawn((
          PbrBundle {
            mesh: mesh.add(make_mesh(vor_poly.clone())),
            material: materials.add(Color::GRAY),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
          },
          Name::new(format!("Poly {}", index)),
        ))
        .insert(Affiliation(LevelCode::Known(KnownLevel::Hub)));
    }
    

    // camera
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(5., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
                projection: Projection::Perspective(PerspectiveProjection::default()),
                camera: Camera {
                    order: PRIMARY_CAMERA_ORDER,
                    ..default()
                },
                ..Default::default()
            },
            MainCamera,
        ))
        .insert(Affiliation(LevelCode::Known(KnownLevel::Hub)));

    // light
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
        .insert(Affiliation(LevelCode::Known(KnownLevel::Hub)));

    // plane
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
        .insert(Affiliation(LevelCode::Known(KnownLevel::Hub)));

    // cube
    commands
        .spawn((
            PbrBundle {
                mesh: mesh
      .add(Mesh::from(Cuboid::from_size(Vec3::new(0.5, 0.5, 0.5)))),
                material: materials.add(Color::GRAY),
                transform: Transform::from_xyz(0., 0.25, 0.),
                ..Default::default()
            },
            Name::new("Cube"),
        ))
        .insert(Affiliation(LevelCode::Known(KnownLevel::Hub)));
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
