#![allow(clippy::module_inception)]

mod egui_frame_preset;
mod menu;
mod game_menu;
mod ui;
mod debug;

use egui_frame_preset::*;
pub use menu::*;
pub use game_menu::*;
pub use ui::*;
pub use debug::*;
