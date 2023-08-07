use bevy::{
    core_pipeline::clear_color::ClearColorConfig, log::LogPlugin, prelude::*,
    render::camera::ScalingMode, window::WindowMode,
};
#[cfg(feature = "inspect")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{prelude::RaycastPickCamera, DefaultPickingPlugins};
use bevy_sprite3d::Sprite3dPlugin;
use bevy_tweening::TweeningPlugin;
use effects::EffectsPlugin;
use game::Game;
use lazy_static::lazy_static;

use locale::LocalePlugin;
use ui::UiPlugin;

mod effects;
mod ext;
mod game;
mod helper;
mod lens;
mod locale;
mod ui;

pub use locale::LocaleLangs;

//
// Launcher Game Enviroment Variables
//
pub const LAUNCHER_TITLE: &str = "Kill Errors";
pub(crate) const HOME_URL: &str = env!("HOME_URL");

//
// Game Enviroment Variables
//
lazy_static! {
    pub(crate) static ref TIME_WAIT_TO_START: u64 = {
        let time = env!("TIME_WAIT_TO_START");
        time.parse::<u64>().unwrap_or(10)
    };
    pub(crate) static ref MAX_BUGS_ON_SCREEN: usize = {
        let time = env!("MAX_BUGS_ON_SCREEN");
        time.parse::<usize>().unwrap_or(30)
    };
}

#[derive(Resource)]
pub(crate) struct OpenLinkResource(pub Box<dyn Fn(&str) + Sync + Send + 'static>);

#[derive(Clone, Default, Debug, Hash, States, PartialEq, Eq)]
pub(crate) enum GameState {
    #[default]
    MainPage,
    Game,
}

pub fn app(
    fullscreen: bool,
    lang: LocaleLangs,
    open_url: impl Fn(&str) + Sync + Send + 'static,
) -> App {
    let mode = if fullscreen {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    };

    let mut app = App::new();
    app.insert_resource(OpenLinkResource(Box::new(open_url)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode,
                        title: LAUNCHER_TITLE.to_string(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        decorations: false,
                        ..default()
                    }),
                    ..default()
                })
                .disable::<LogPlugin>(),
        )
        .add_plugins(DefaultPickingPlugins.build())
        .add_plugins((Sprite3dPlugin, TweeningPlugin));
    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::new());
    app.add_state::<GameState>()
        .insert_resource(lang)
        .add_systems(Startup, setup_camera)
        .add_plugins((LocalePlugin, UiPlugin, EffectsPlugin, Game));

    app
}

fn setup_camera(mut cmd: Commands) {
    cmd.spawn((
        Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(25.),
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 25.),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::rgb_u8(227, 227, 227)),
                ..default()
            },
            ..default()
        },
        RaycastPickCamera::default(),
    ));

    cmd.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(8., 16., 8.),
        ..default()
    });
}
