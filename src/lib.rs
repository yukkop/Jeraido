#![allow(clippy::module_inception)]

mod actor;
mod component;
mod controls;
mod level;
mod lobby;
mod settings;
mod sound;
mod ui;
mod util;
mod world;

#[cfg(all(debug_assertions, feature = "dev"))]
pub mod editor;
pub mod core;

pub const ASSET_DIR: &str = "asset";
