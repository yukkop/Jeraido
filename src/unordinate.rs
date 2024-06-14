use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, VertexAttributeValues};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy_controls::{
    contract::InputsContainer,
    plugin::ControlsPlugin,
    resource::{
        ActivationMode, ActivationOptions, AxisName, Binding, BindingCondition, BindingConfig,
        Bindings, ButtonCombination, Controls, InputType, InputValue, MouseInput, OptionsMode,
        PlayerActions,
    },
};
use bevy_controls_derive::{Action, GameState};
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;
use strum_macros::EnumIter;

const CUBE_SIZE: f32 = 1.;
const HALF_CUBE_SIZE: f32 = CUBE_SIZE / 2.;
const CUBE_SUBSTITUTIONS: i32 = 4;
const SUBSTITUTIONS_SIZE: f32 = CUBE_SIZE / CUBE_SUBSTITUTIONS as f32;
const SUBSTITUTIONS_SIZE_PADDING: f32 = SUBSTITUTIONS_SIZE / 100. * 10.;
const NATURAL_SUBSTITUTIONS_SIZE: f32 = SUBSTITUTIONS_SIZE; // / 100. * 90.;
const HALF_SUBSTITUTIONS_SIZE: f32 = SUBSTITUTIONS_SIZE / 2.;

#[derive(PartialEq, Eq, Hash, EnumIter, Clone, Copy, Debug, Action)]
enum MyAction {
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
    MouseHorizontal,
    MouseVertical,
}

#[derive(States, PartialEq, Eq, Clone, Hash, Debug, Default, GameState)]
enum MyGameState {
    #[default]
    InGame,
    Menu,
}

#[derive(Resource, Default, Clone, Debug)]
struct MyInputsContainer {
    // When the game does not provide multiplayer, one field is enough
    player_inputs: PlayerActions<MyAction>,
}

impl InputsContainer<MyAction> for MyInputsContainer {
    fn iter_inputs<'a>(&'a self) -> Box<dyn Iterator<Item = &'a PlayerActions<MyAction>> + 'a> {
        todo!()
    }

    fn me<'a>(&'a self) -> Option<&'a PlayerActions<MyAction>> {
        Some(&self.player_inputs)
    }

    fn me_mut<'a>(&'a mut self) -> Option<&'a mut PlayerActions<MyAction>> {
        Some(&mut self.player_inputs)
    }
}

#[derive(Component)]
struct Sliceble;
#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EditorPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            ControlsPlugin::<MyAction, MyInputsContainer, MyGameState>::new(
                Controls::<MyAction, MyGameState>::new()
                    .with(
                        MyAction::Left,
                        BindingConfig::from_vec(vec![Binding::from_single(InputType::Keyboard(
                            KeyCode::KeyA,
                        ))]),
                    )
                    .with(
                        MyAction::Back,
                        BindingConfig::from_vec(vec![Binding::from_single(InputType::Keyboard(
                            KeyCode::KeyS,
                        ))]),
                    )
                    .with(
                        MyAction::Forward,
                        BindingConfig::from_vec(vec![Binding::from_single(InputType::Keyboard(
                            KeyCode::KeyW,
                        ))]),
                    )
                    .with(
                        MyAction::Right,
                        BindingConfig::from_vec(vec![Binding::from_single(InputType::Keyboard(
                            KeyCode::KeyD,
                        ))]),
                    )
                    .with(
                        MyAction::Up,
                        BindingConfig::from_bind(Binding::from_single(InputType::Keyboard(
                            KeyCode::Space,
                        ))),
                    )
                    .with(
                        MyAction::MouseHorizontal,
                        BindingConfig::from_bind(Binding::from_single(InputType::Mouse(
                            MouseInput::Axis(AxisName::Horizontal),
                        ))),
                    )
                    .with(
                        MyAction::MouseVertical,
                        BindingConfig::from_bind(Binding::from_single(InputType::Mouse(
                            MouseInput::Axis(AxisName::Vertical),
                        ))),
                    )
                    .with(
                        MyAction::Down,
                        BindingConfig::from_bind(Binding::from_single(InputType::Keyboard(
                            KeyCode::ShiftLeft,
                        ))),
                    )
                    .build(),
            ),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, slice)
        .run();
}

fn slice(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<
        (
            Entity,
            &GlobalTransform,
            &Handle<Mesh>,
            &Handle<StandardMaterial>,
        ),
        With<Sliceble>,
    >,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (entity, global_transform, mesh_handle, material_handle) in query.iter() {
            let Some(mesh) = meshes.get(mesh_handle) else {
                panic!()
            };
            let start_point = global_transform.translation()
                - Vec3::new(HALF_CUBE_SIZE, HALF_CUBE_SIZE, HALF_CUBE_SIZE);
            commands.entity(entity).despawn();

            let mesh = Mesh::from(shape::Cube {
                size: NATURAL_SUBSTITUTIONS_SIZE,
            });

            for x in 0..CUBE_SUBSTITUTIONS {
                for y in 0..CUBE_SUBSTITUTIONS {
                    for z in 0..CUBE_SUBSTITUTIONS {
                        commands.spawn((
                            PbrBundle {
                                mesh: meshes.add(mesh.clone()),
                                material: material_handle.clone(),
                                transform: Transform::from_xyz(
                                    HALF_SUBSTITUTIONS_SIZE
                                        + start_point.x
                                        + SUBSTITUTIONS_SIZE * x as f32,
                                    HALF_SUBSTITUTIONS_SIZE
                                        + start_point.y
                                        + SUBSTITUTIONS_SIZE * y as f32,
                                    HALF_SUBSTITUTIONS_SIZE
                                        + start_point.z
                                        + SUBSTITUTIONS_SIZE * z as f32,
                                ),
                                ..default()
                            },
                            RigidBody::Dynamic,
                            Collider::cuboid(
                                NATURAL_SUBSTITUTIONS_SIZE / 2.,
                                NATURAL_SUBSTITUTIONS_SIZE / 2.,
                                NATURAL_SUBSTITUTIONS_SIZE / 2.,
                            ),
                            Ccd::enabled(), // TODO
                        ));
                    }
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh::from(shape::Cube { size: CUBE_SIZE });

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh.clone()),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Sliceble,
        RigidBody::Dynamic,
        Collider::cuboid(HALF_CUBE_SIZE, HALF_CUBE_SIZE, HALF_CUBE_SIZE),
    ));

    let circle_mesh_handler = meshes.add(Circle::new(4.0));

    // circular base
    commands.spawn((
        PbrBundle {
            mesh: circle_mesh_handler.clone(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            ..default()
        },
        RigidBody::Fixed,
        Collider::from_bevy_mesh(
            &Mesh::from(Circle::new(4.0)),
            &ComputedColliderShape::TriMesh,
        )
        .unwrap(),
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn player_movement(
    mut query: Query<&Transform, With<Player>>,
    inputs_container: Res<MyInputsContainer>,
) {
    for transform in query.iter_mut() {
        //let player_inputs = inputs_container.me().expect("This is bad");
        //player_inputs.get(MyAction::Back)
        //if player_inputs
        //  .get_just_pressed(MyAction::Up)
        //  .unwrap_or(false)
        //{
        //}
    }
}
