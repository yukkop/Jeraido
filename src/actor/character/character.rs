use std::f32::consts::PI;

use crate::component::{AxisName, DespawnReason, NoclipDuration, Respawn};
use crate::extend_commands;
use crate::lobby::host::{generate_player_color, server_update_system};
use crate::lobby::{Character, InputType, PlayerInputs};
use crate::lobby::{LobbyState, PlayerId, PlayerView};
use crate::world::SpawnPoint;
use crate::world::MainCamera;
use crate::world::Me;
use bevy::ecs::query::QueryFilter;
use bevy::{ecs::system::EntityCommands, prelude::*};
use bevy_rapier3d::dynamics::Velocity;
use serde::{Deserialize, Serialize};

pub const PLAYER_MOVE_SPEED: f32 = 0.07;
pub const PLAYER_SIZE: f32 = 2.0;
const SHIFT_ACCELERATION: f32 = 2.0;
const SENSITIVITY: f32 = 0.5;
const JUMP_HEIGHT_MULTIPLICATOR: f32 = 1.1;

const DEFAULT_CAMERA_DISTANCE: f32 = 20.;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct TiedCamera(Entity);

#[derive(Component, Debug)]
struct JumpHelper {
    last_viable_normal: Vec3,
}

pub struct CharacterPlugins;

impl Plugin for CharacterPlugins {
    fn build(&self, app: &mut App) {
        app
        // .add_systems(
        //     FixedUpdate,
        //     (move_characters, update_jump_normals).run_if(
        //         not(in_state(LobbyState::None)).and_then(not(in_state(LobbyState::Client))),
        //     ),
        // )
        // .add_systems(
        //     Update,
        //     (jump, rotate_camera).run_if(
        //         not(in_state(LobbyState::None)).and_then(not(in_state(LobbyState::Client))),
        //     ),
        // )
        // .add_systems(
        //     Last,
        //     fire.after(server_update_system).run_if(
        //         not(in_state(LobbyState::None)).and_then(not(in_state(LobbyState::Client))),
        //     ),
        // )
        .add_systems(
            PostUpdate,
            tied_camera_follow.run_if(not(in_state(LobbyState::None))),
        );
    }
}

fn tied_camera_follow(
    mut tied_camera_query: Query<(&TiedCamera, &Children, &mut Transform)>,
    mut camera_query: Query<&mut Transform, (Without<TiedCamera>, With<Camera>)>,
    view_direction_query: Query<&PlayerView, With<Me>>,
    transform_query: Query<&Transform, (Without<TiedCamera>, Without<Camera>)>,
) {
    for (TiedCamera(target), children, mut transform) in tied_camera_query.iter_mut() {
        if let Ok(target_transform) = transform_query.get(*target) {
            transform.translation = target_transform.translation + Vec3::Y * 2.;
            if let Ok(view) = view_direction_query.get_single() {
                transform.rotation = view.direction;
                if let Some(child) = children.iter().next() {
                    if let Ok(mut camera_transform) = camera_query.get_mut(*child) {
                        camera_transform.translation = view.distance * Vec3::Z;
                    }
                }
            }
        } else {
            warn!(
                "Tied camera cannot follow object ({:?}) without transform",
                target
            )
        }
    }
}

// TODO jump for rapiera
//fn update_jump_normals(
//    mut query: Query<(&mut JumpHelper, Entity, &GlobalTransform)>,
//    collisions: Res<Collisions>,
//) {
//    let mut direction_vector = Vec3::ZERO;
//    for (mut jump_direction, player_entity, transform) in query.iter_mut() {
//        for collision in collisions.collisions_with_entity(player_entity) {
//            for manifold in collision.manifolds.iter() {
//                for contact in manifold.contacts.iter() {
//                    direction_vector += transform.translation() - contact.point1;
//                }
//            }
//        }
//        jump_direction.last_viable_normal = direction_vector.normalize_or_zero();
//    }
//}

//fn jump(
//    mut query: Query<(
//        &mut LinearVelocity,
//        &PlayerView,
//        &mut PlayerInputs,
//        Entity,
//        &JumpHelper,
//    )>, /* , time: Res<Time> */
//    gravity: Res<Gravity>,
//    collisions: Res<Collisions>,
//) {
//    for (mut linear_velocity, view_direction, mut player_inputs, player_entity, jump_direction) in
//        query.iter_mut()
//    {
//        let jumped = player_inputs.is_input_changed_to_true_and_set_to_false(InputType::Jump);
//
//        let dx = (player_inputs.get().right as i8 - player_inputs.get().left as i8) as f32;
//        let dy = (player_inputs.get().down as i8 - player_inputs.get().up as i8) as f32;
//
//        let local_x = view_direction.direction.mul_vec3(Vec3::X);
//        let local_y = view_direction.direction.mul_vec3(Vec3::Z);
//
//        if jumped
//            && collisions
//                .collisions_with_entity(player_entity)
//                .next()
//                .is_some()
//        {
//            **linear_velocity += ((jump_direction.last_viable_normal + local_x * dx + local_y * dy)
//                    .normalize_or_zero())
//                    * (-gravity.0.y * 2.0 * PLAYER_SIZE).sqrt() // sqrt(2gh)
//                    * JUMP_HEIGHT_MULTIPLICATOR;
//            log::debug!("{:?}", jump_direction.last_viable_normal);
//        }
//    }
//}

fn move_characters(
    mut query: Query<(&mut Velocity, &PlayerView, &PlayerInputs)>, /* , time: Res<Time> */
) {
    for (mut velocity, view_direction, input) in query.iter_mut() {
        let input = input.get();
        let dx = (input.right as i8 - input.left as i8) as f32;
        let dy = (input.down as i8 - input.up as i8) as f32;

        // convert axises to global
        let view_direction_x = view_direction.direction.mul_vec3(Vec3::X);
        let view_direction_y = view_direction.direction.mul_vec3(Vec3::Z);

        // never use delta time in fixed update !!!
        let shift_acceleration = SHIFT_ACCELERATION.powf(input.sprint as i32 as f32);

        // move by x axis
        velocity.linvel.x += dx * PLAYER_MOVE_SPEED * view_direction_x.x * shift_acceleration;
        velocity.linvel.z += dx * PLAYER_MOVE_SPEED * view_direction_x.z * shift_acceleration;

        // move by y axis
        velocity.linvel.x += dy * PLAYER_MOVE_SPEED * view_direction_y.x * shift_acceleration;
        velocity.linvel.z += dy * PLAYER_MOVE_SPEED * view_direction_y.z * shift_acceleration;
    }
}

#[allow(clippy::type_complexity)]
fn rotate_camera(
    mut query: Query<(
        &mut PlayerView,
        &Transform,
        &PlayerInputs,
        //Option<&mut RayCaster>,
        //Option<&RayHits>,
    )>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_seconds();
    for (mut view, transform, input) in query.iter_mut() {
        let input = input.get();

        // camera turn
        let rotation = Quat::from_rotation_y(input.turn_horizontal * SENSITIVITY * delta_seconds);
        // global rotation (!ORDER OF MULTIPLICATION MATTERS!)
        view.direction = rotation * view.direction;

        let rotation = Quat::from_rotation_x(input.turn_vertical * SENSITIVITY * delta_seconds);
        // local rotation (!ORDER OF MULTIPLICATION MATTERS!)
        view.direction *= rotation;

        view.distance = DEFAULT_CAMERA_DISTANCE;

        // TODO:
        // let ray_pos = Vec3::new(1.0, 2.0, 3.0);
        // let ray_dir = Vec3::new(0.0, 1.0, 0.0);
        // let max_toi = 4.0;
        // let solid = true;
        // let filter = QueryFilter::default();
        // //rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) 
        // 
        // if let (Some(hits), Some(mut ray)) = (hits, ray) {
        //     let h = transform.rotation.conjugate();
        //     let start_point = h.mul_vec3(Vec3::Y * 2.);
        //     let offset = h
        //         .mul_vec3(view.direction.mul_vec3(DEFAULT_CAMERA_DISTANCE * Vec3::Z))
        //         // need to normalize ray.direction to time_of_impact work correctly
        //         .normalize();
        //     ray.origin = start_point;
        //     ray.direction = offset;
        // 
        //     if let Some(firs_hit) = hits.iter_sorted().next() {
        //         if firs_hit.time_of_impact < DEFAULT_CAMERA_DISTANCE {
        //             view.distance = firs_hit.time_of_impact;
        //         }
        //     }
        // }
    }
}

extend_commands!(
  spawn_character(player_id: PlayerId, color: Color, spawn_point: Vec3),
  |world: &mut World, entity_id: Entity, player_id: PlayerId, color: Color, spawn_point: Vec3| {

    let mesh = world
      .resource_mut::<Assets<Mesh>>()
      // TODO: Have a resource with shared mesh list instead of adding meshes each time
      .add(Mesh::from(shape::Cube { size: PLAYER_SIZE }));
    let material = world
      .resource_mut::<Assets<StandardMaterial>>()
      .add(color);

      // some raycast magic
    let start_point = Vec3::Y * 2.;
    let offset = Vec3::new(0., 0., DEFAULT_CAMERA_DISTANCE);

    world
        .entity_mut(entity_id)
        .insert((
            PbrBundle {
            mesh,
            material,
            ..Default::default()
            },
            // TODO: RayCaster::new(start_point, offset),
            JumpHelper{last_viable_normal: Vec3::Y},
            Respawn::new((
                DespawnReason::More(200., AxisName::Y),
                DespawnReason::Less(-10., AxisName::Y),
                DespawnReason::More(100., AxisName::X),
                DespawnReason::Less(-100., AxisName::X),
                DespawnReason::More(100., AxisName::Z),
                DespawnReason::Less(-100., AxisName::Z)
            ),
            SpawnPoint::new(spawn_point),
            NoclipDuration::Timer(10.)),
            PlayerInputs::default(),
            Character { id: player_id },
            PlayerView::new(Quat::default(), 325_f32.sqrt()),
            Name::new(format!("Character:{:#?}", player_id)),
            // PhysicsOptimalTrace::new(0.5, 0.05, color, PLAYER_SIZE / 2.),
        ))
        // TODO: 
        //.insert((
        //    Friction::new(0.4),
        //    PhysicsBundle::from_rigid_body(RigidBody::Dynamic),
        //    //TODO colider
        //))
        ;
  }
);

extend_commands!(
  spawn_character_shell(player_id: PlayerId, color: Color, spawn_point: Vec3),
  |world: &mut World, entity_id: Entity, player_id: PlayerId, color: Color, spawn_point: Vec3| {

    let mesh = world
      .resource_mut::<Assets<Mesh>>()
      // TODO: Have a resource with shared mesh list instead of adding meshes each time
      .add(Mesh::from(shape::Cube { size: PLAYER_SIZE }));
    let material = StandardMaterial {
      base_color: color,
      ..default()
    };
    let material = world
      .resource_mut::<Assets<StandardMaterial>>()
      .add(material);

    world
     .entity_mut(entity_id)
     .insert((
       PbrBundle {
          mesh,
          material,
          transform: Transform::from_xyz(spawn_point.x, spawn_point.y, spawn_point.z),
          ..Default::default()
       },
        // TransformOptimalTrace::new(0.5, 0.05, color, PLAYER_SIZE / 2.),
        PlayerInputs::default(),
        Name::new(format!("Character:{:#?}", player_id)),
        PlayerView::new(Quat::default(), 325_f32.sqrt())));
  }
);

extend_commands!(
  spawn_tied_camera(target: Entity),
  |world: &mut World, entity_id: Entity, target: Entity| {
    world
      .spawn(
          NodeBundle{
              style: Style{
                  width: Val::Px(2.0),
                  height: Val::Px(2.0),
                  align_self: AlignSelf::Center,
                  justify_self: JustifySelf::Center,
                  ..default()
              },
              background_color: Color::rgb(1.0, 0.0, 0.0).into(),
              ..default()
          }
      );
    world
      .entity_mut(entity_id)
      .insert((
        // TODO find light prd without mesh
        PbrBundle::default(),
        TiedCamera(target),
        Name::new("TiedCamera"),
      ))
      .with_children(|parent| {
        // spawn tied camera
        parent.spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., DEFAULT_CAMERA_DISTANCE)).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            MainCamera,
        ));
      });
  }
);
