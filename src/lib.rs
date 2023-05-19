use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::WindowMode};
#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use lazy_static::lazy_static;

use ui::UiPlugin;

mod lens;
mod ui;

// Launcher Game Enviroment Variables
pub const LAUNCHER_TITLE: &str = "Kill Errors";
pub(crate) const HOME_URL: &str = env!("HOME_URL");

// Game Enviroment Variables
lazy_static! {
    pub(crate) static ref TIME_WAIT_TO_START: u64 = {
        let time = env!("TIME_WAIT_TO_START");
        time.parse::<u64>().unwrap_or(10)
    };
}

#[derive(Clone, Resource)]
pub(crate) struct OpenLinkResource(pub Box<fn(&str)>);

pub fn app(fullscreen: bool, open_url: fn(&str)) -> App {
    let mode = if fullscreen {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    };

    let mut app = App::new();
    app.insert_resource(OpenLinkResource(Box::new(open_url)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode,
                title: LAUNCHER_TITLE.to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                present_mode: bevy::window::PresentMode::AutoVsync,
                decorations: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(TweeningPlugin);
    #[cfg(feature = "inspect")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(setup_camera).add_plugin(UiPlugin);

    app
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb_u8(227, 227, 227)),
        },
        ..default()
    });
}
