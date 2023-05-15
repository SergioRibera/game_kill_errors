use bevy::{prelude::*, window::WindowMode};

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
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            present_mode: bevy::window::PresentMode::AutoVsync,
            decorations: false,
            ..default()
        }),
        ..default()
    }));

    app
}

