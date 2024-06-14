use bevy::{app::{App, Plugin}, ecs::{system::Resource, schedule::States}, a11y::accesskit::Action, input::keyboard::KeyCode};
use bevy_controls::{resource::{PlayerActions, BindingConfig, Binding, InputType, AxisName, MouseInput, Controls}, contract::InputsContainer, plugin::ControlsPlugin};
use bevy_controls_derive::{GameState, Action};
use bevy_rapier3d::{plugin::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};
use strum_macros::EnumIter;

#[derive(PartialEq, Eq, Hash, EnumIter, Clone, Copy, Debug, Action)]
pub enum CoreAction {
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
pub enum CoreGameState {
    #[default]
    InGame,
    Menu,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct Lobby {
    // When the game does not provide multiplayer, one field is enough
    player_inputs: PlayerActions<CoreAction>,
}

impl InputsContainer<CoreAction> for Lobby {
    fn iter_inputs<'a>(&'a self) -> Box<dyn Iterator<Item = &'a PlayerActions<CoreAction>> + 'a> {
        todo!()
    }

    fn me<'a>(&'a self) -> Option<&'a PlayerActions<CoreAction>> {
        Some(&self.player_inputs)
    }

    fn me_mut<'a>(&'a mut self) -> Option<&'a mut PlayerActions<CoreAction>> {
        Some(&mut self.player_inputs)
    }
}

/// Main plugin of the game
pub struct ControlsPlugins;

impl Plugin for ControlsPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ControlsPlugin::<CoreAction, Lobby, CoreGameState>::new(
                Controls::<CoreAction, CoreGameState>::new()
                    .with(
                        CoreAction::Left,
                        BindingConfig::from_vec(vec![Binding::from_single(InputType::Keyboard(
                            KeyCode::KeyA,
                        ))]),
                    )
                    .with(
                        CoreAction::Back,
                        BindingConfig::from_vec(vec![Binding::from_single(InputType::Keyboard(
                            KeyCode::KeyS,
                        ))]),
                    )
                    .with(
                        CoreAction::Forward,
                        BindingConfig::from_vec(vec![Binding::from_single(InputType::Keyboard(
                            KeyCode::KeyW,
                        ))]),
                    )
                    .with(
                        CoreAction::Right,
                        BindingConfig::from_vec(vec![Binding::from_single(InputType::Keyboard(
                            KeyCode::KeyD,
                        ))]),
                    )
                    .with(
                        CoreAction::Up,
                        BindingConfig::from_bind(Binding::from_single(InputType::Keyboard(
                            KeyCode::Space,
                        ))),
                    )
                    .with(
                        CoreAction::MouseHorizontal,
                        BindingConfig::from_bind(Binding::from_single(InputType::Mouse(
                            MouseInput::Axis(AxisName::Horizontal),
                        ))),
                    )
                    .with(
                        CoreAction::MouseVertical,
                        BindingConfig::from_bind(Binding::from_single(InputType::Mouse(
                            MouseInput::Axis(AxisName::Vertical),
                        ))),
                    )
                    .with(
                        CoreAction::Down,
                        BindingConfig::from_bind(Binding::from_single(InputType::Keyboard(
                            KeyCode::ShiftLeft,
                        ))),
                    )
                    .build(),
            ),
        ));
    }
}
