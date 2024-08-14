use std::env;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::winit::WinitWindows;
use bevy_egui::EguiPlugin;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use jeraido::core::CorePlugins;
use jeraido::ASSET_DIR;
use winit::window::Icon;
#[cfg(all(debug_assertions, feature = "devtools"))]
use jeraido::DEBUG;

/// default value for logging
///
/// wgpu_core fluds the logs on info level therefore we need to set it to error
const LOG_FILTER: &str = "info,wgpu_core=error";
/// The path to the icon
const ICON_PATH: &str = "icon.png";

/// The name of the application
const APP_NAME: &str = "pih-pah";

lazy_static::lazy_static! {
    /// The current version of the application
    pub static ref VERSION: String = format!("{}.{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"), env!("CARGO_PKG_VERSION_PATCH"));

    /// The name of the application with the version
    pub static ref VERSIONED_APP_NAME: String = format!("{APP_NAME} v{}", *VERSION);
}

fn main() {
    let mut app = App::new();

    let asset_plugin = AssetPlugin {
        file_path: ASSET_DIR.into(),
        ..default()
    };

     #[cfg(not(debug_assertions))]
     let level = Level::INFO;
     #[cfg(debug_assertions)]
     let level = Level::DEBUG;

    let log_plugin = LogPlugin {
      level,
      filter: LOG_FILTER.to_string(),
      update_subscriber: None,
    };

    /// Build the app with the default plugins
    fn default_build(app: &mut App, asset_plugin: AssetPlugin, log_plugin: LogPlugin) -> &mut App {
        let window_plugin_override = WindowPlugin {
            primary_window: Some(Window {
                title: VERSIONED_APP_NAME.clone(),
                //fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        };
        app.add_plugins((
            DefaultPlugins.set(asset_plugin).set(log_plugin).set(window_plugin_override),
            EguiPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ))
    }

    #[cfg(not(feature = "devtools"))]
    default_build(&mut app, asset_plugin, log_plugin);

    #[cfg(all(debug_assertions, feature = "devtools"))]
    if !*DEBUG {
        default_build(&mut app, asset_plugin, log_plugin);
    } else {
        use bevy::window::PresentMode;
        use bevy::window::WindowResolution;
        use bevy_rapier3d::render::RapierDebugRenderPlugin;
        use jeraido::editor::EditorPlugins;

        let window_plugin_override = WindowPlugin {
            primary_window: Some(Window {
                title: VERSIONED_APP_NAME.clone(),
                resolution: WindowResolution::default(),
                present_mode: PresentMode::AutoNoVsync,
                // Tells wasm to resize the window according to the available canvas
                //fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        };
        app.add_plugins((
            DefaultPlugins
              .set(asset_plugin)
              .set(log_plugin)
              .set(window_plugin_override),
            EguiPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            EditorPlugins,
        ));
    }

    // it can be difficult to make physics undependent from the frame rate
    // but we cannot use FixedUpdate because it is not supported by bevy_xpbd_3d as well as
    app.add_systems(Startup, set_window_icon)
        .add_plugins(CorePlugins);

    info!("Starting {APP_NAME} v{}", *VERSION);

    app.run();
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    let exe_path = env::current_exe().expect("Failed to find executable path");
    let exe_dir = exe_path
        .parent()
        .expect("Failed to find executable directory");
    let (icon_rgba, icon_width, icon_height) = {
        if let Ok(image) = image::open(exe_dir.join(ICON_PATH)) {
            let image = image.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        } else {
            // TODO load default icon from url
            warn!("Failed to load icon");
            return;
        }
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
