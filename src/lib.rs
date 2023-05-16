use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::WindowMode};
#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use ui::UiPlugin;

mod ui;

pub const LAUNCHER_TITLE: &str = "Kill Errors";

pub fn app(fullscreen: bool) -> App {
    let mode = if fullscreen {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
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
    }));
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
