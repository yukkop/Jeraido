use crate::ui::menu::MenuPlugins;
use crate::util::i18n::{trans, Uniq};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_egui::egui::FontId;
use std::sync::Arc;

use super::GameMenuPlugins;

#[derive(Debug, Clone, Copy, Resource, PartialEq, Deref, DerefMut)]
pub struct ViewportRect(egui::Rect);

impl Default for ViewportRect {
    fn default() -> Self {
        Self(egui::Rect::from_min_size(
            Default::default(),
            Default::default(),
        ))
    }
}

impl ViewportRect {
    pub fn set(&mut self, rect: egui::Rect) {
        self.0 = rect;
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MouseGrabState {
    Enable,
    #[default]
    Disable,
}

impl MouseGrabState {
    pub fn toggle(&mut self) -> Self {
        match self {
            MouseGrabState::Enable => *self = MouseGrabState::Disable,
            MouseGrabState::Disable => *self = MouseGrabState::Enable,
        }
        *self
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum UiState {
    #[default]
    Menu,
    GameMenu,
}

pub struct UiPlugins;

impl Plugin for UiPlugins {
    fn build(&self, app: &mut App) {
        app.insert_state(UiState::default())
            .insert_state(MouseGrabState::default())
            .init_resource::<ViewportRect>()
            .add_plugins((MenuPlugins, GameMenuPlugins))
            .add_systems(OnEnter(MouseGrabState::Enable), grab_mouse_on)
            .add_systems(OnEnter(MouseGrabState::Disable), grab_mouse_off)
            // Not to friecventrly?
            .add_systems(Update, frame_rect);
    }
}

// TODO: forgoten realization, maybe reaction on window resize
fn from_window(mut windows: Query<&Window>, mut ui_frame_rect: ResMut<ViewportRect>) {
    let window = windows.single_mut();
    let window_size = egui::vec2(window.width(), window.height());

    ui_frame_rect.set(egui::Rect::from_min_size(Default::default(), window_size));
}

#[cfg(not(all(debug_assertions, feature = "dev")))]
pub fn frame_rect(windows: Query<&Window>, ui_frame_rect: ResMut<ViewportRect>) {
    from_window(windows, ui_frame_rect);
}

#[cfg(all(debug_assertions, feature = "dev"))]
use {crate::DEBUG, bevy_editor_pls::editor::Editor};

#[cfg(all(debug_assertions, feature = "dev"))]
pub fn frame_rect(
    windows: Query<&Window>,
    editor: Res<Editor>,
    mut ui_frame_rect: ResMut<ViewportRect>,
) {
    if !*DEBUG {
        from_window(windows, ui_frame_rect);
    } else {
        if editor.active() {
            ui_frame_rect.set(editor.viewport());
        } else {
            from_window(windows, ui_frame_rect);
        }
    }
}

pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::WidgetText {
    egui::WidgetText::RichText(egui::RichText::new(trans(text.into(), uniq)).font(font.clone()))
}

//pub fn rich_text(text: impl Into<Arc<String>>, uniq: Uniq, font: &FontId) -> egui::RichText {
//    egui::RichText::new(trans(text.into(), uniq)).font(font.clone())
//}

fn grab_mouse_on(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

fn grab_mouse_off(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;
}
