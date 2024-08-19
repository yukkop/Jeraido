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

fn find_min_max(points: &Vec<Vec3>) -> (f32, f32, f32, f32) {
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_z = f32::MAX;
    let mut max_z = f32::MIN;

    for p in points {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.x > max_x {
            max_x = p.x;
        }
        if p.z < min_z {
            min_z = p.z;
        }
        if p.z > max_z {
            max_z = p.z;
        }
    }

    (min_x, max_x, min_z, max_z)
}

fn calculate_normal(v0: Vec3, v1: Vec3, v2: Vec3) -> Vec3 {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    edge1.cross(edge2).normalize()
}

fn extrude_to_mesh(polygon: Vec<Point>) -> Mesh {
    // Given data: Vec<Point>
    let points: Vec<Vec3> = polygon.into_iter().map(|e| {
        Vec3 {x: e.x.0 as f32, y: 0., z: e.y.0 as f32}
    }).collect();

    // Ensure you have at least 3 points to form a plane
    if points.len() < 3 {
        panic!("Expected at least 3 points to form a plane");
    }

    
    let extrusion_depth = 1.0;

    // Create the top face by offsetting the points
    let top_face: Vec<Vec3> = points.iter().map(|p| *p + Vec3::new(0.0, extrusion_depth, 0.0)).collect();
    
    // Combine top and bottom vertices
    let mut vertices = points.clone();
    vertices.extend(top_face.iter());

    // Calculate side faces' indices
    let mut indices = Vec::new();

    let num_points = points.len();
    
    // Top face indices (CCW)
    for i in 1..num_points - 1 {
        indices.push(num_points as u32); // First vertex of the top face
        indices.push((num_points + i + 1) as u32);
        indices.push((num_points + i) as u32);
    }

    // Bottom face indices (CW)
    for i in 1..num_points - 1 {
        indices.push(0 as u32);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }

    // Side faces
    for i in 0..num_points {
        let next_i = (i + 1) % num_points;

        indices.push(i as u32);
        indices.push((num_points + i) as u32);
        indices.push(next_i as u32);

        indices.push(next_i as u32);
        indices.push((num_points + i) as u32);
        indices.push((num_points + next_i) as u32);
    }

    // Generate normals
    let top_normal = Vec3::new(0.0, 1.0, 0.0); // Since it's extruded upwards in the y-axis
    let bottom_normal = Vec3::new(0.0, -1.0, 0.0); // Opposite direction for the bottom face

    let mut vertex_normals: Vec<Vec3> = vec![Vec3::ZERO; vertices.len()];

    // Assign the top and bottom face normals to the respective vertices
    for i in 0..points.len() {
        // Bottom face
        vertex_normals[i] += bottom_normal;
        
        // Top face
        vertex_normals[i + points.len()] += top_normal;
    }

    for i in 0..points.len() {
      // Points defining the side face
      let v0 = points[i];
      let v1 = points[(i + 1) % points.len()];
      let v2 = top_face[i];
      
      // Calculate the normal for the side face
      let side_normal = calculate_normal(v0, v1, v2);
      
      // Assign this normal to the vertices of the side face
      vertex_normals[i] += side_normal; // Bottom vertex i
      vertex_normals[(i + 1) % points.len()] += side_normal; // Bottom vertex i+1
      vertex_normals[i + points.len()] += side_normal; // Top vertex i
      vertex_normals[(i + 1) % points.len() + points.len()] += side_normal; // Top vertex i+1
    }
     
    // Assuming the same order for side faces as for vertices
    for i in 0..points.len() {
        let side_normal = calculate_normal(points[i], top_face[i], top_face[(i + 1) % points.len()]);
        
        // Add this normal to the corresponding vertices
        vertex_normals[i] += side_normal; // Bottom vertex
        vertex_normals[i + points.len()] += side_normal; // Top vertex
    }
    
    // Normalize the normals to make them unit vectors
    for normal in &mut vertex_normals {
        *normal = normal.normalize();
    }
                                                                
    let (min_x, max_x, min_z, max_z) = find_min_max(&points);

    // Generate UVs for the bottom face
    let bottom_uvs: Vec<Vec2> = points.iter().map(|p| {
        Vec2::new(
            (p.x - min_x) / (max_x - min_x), // Normalize x to [0, 1]
            (p.z - min_z) / (max_z - min_z)  // Normalize z to [0, 1]
        )
    }).collect();
    
    // Generate UVs for the top face
    let top_uvs: Vec<Vec2> = bottom_uvs.clone();
    
    // Combine UVs for both top and bottom faces
    let mut uvs = bottom_uvs;
    uvs.extend(top_uvs.iter());

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vertex_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
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
    // A triangle using vertices 0, 2, and 1. 
    // NOTE: order matters. [0, 1, 2] will be flipped upside down,
    // and you won't see it from behind! 
    // mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));
    let mut indices = Vec::new();
    for i in 1..points.len() - 1 {
        indices.push(0 as u32);
        indices.push((i + 1) as u32);
        indices.push(i as u32);
    }

    // Generate a normal (assuming points lie on a plane)
    let normal = (points[1] - points[0]).cross(points[2] - points[0]).normalize();
    let normals = vec![normal * -1.; points.len()];
    log::debug!("normals: {:#?}", normals);

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
    const BOX_SIZE: f64 = 40.;
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
            mesh: mesh.add(extrude_to_mesh(vor_poly.clone())),
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
