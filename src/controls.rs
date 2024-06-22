use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        schedule::{NextState, State},
        system::{Res, ResMut},
    },
    input::keyboard::KeyCode,
};
use bevy_controls::{
    contract::InputsContainer,
    plugin::ControlsPlugin,
    resource::{Binding, BindingCondition, BindingConfig, Controls, InputType},
};

use crate::{
    core::{CoreAction, CoreGameState},
    lobby::Lobby,
    ui::{GameMenuActionState, MouseGrabState},
};

/// Main plugin of the game
pub struct ControlsPlugins;

impl Plugin for ControlsPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, in_game_menu)
            .add_plugins((ControlsPlugin::<CoreAction, Lobby, CoreGameState>::new(
                Controls::<CoreAction, CoreGameState>::new()
                    .with(
                        CoreAction::InGameMenu,
                        BindingConfig::from_vec(vec![Binding::from_single(InputType::Keyboard(
                            KeyCode::Escape,
                        ))
                        .with_condition(BindingCondition::InGameState(CoreGameState::InGame))]),
                    )
                    .build(),
            ),));
    }
}

fn in_game_menu(
    inputs_container: Res<Lobby>,
    mut next_state_mouse_grab: ResMut<NextState<MouseGrabState>>,
    mouse_grab_state: Res<State<MouseGrabState>>,
    mut next_state_game_menu_action: ResMut<NextState<GameMenuActionState>>,
    game_menu_action: Res<State<GameMenuActionState>>,
) {
    let player_inputs = inputs_container.me().expect("This is bad");

    if player_inputs
        .get_just_pressed(CoreAction::InGameMenu)
        .unwrap_or(false)
    {
        log::debug!("escape");
        next_state_game_menu_action.set(game_menu_action.get().clone().toggle());
        next_state_mouse_grab.set(mouse_grab_state.get().clone().toggle());
    }
}
