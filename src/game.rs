mod components;
mod systems;

use bevy::prelude::*;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::prelude::*;

use components::*;
use systems::*;

pub use components::ScoreText;

use crate::GameState;

// Debug Proyect
#[cfg(debug_assertions)]
pub const CRAB_SCORES: &[u64] = &[10, 11, 12, 13, 14, 15];

// Release Proyect
#[cfg(not(debug_assertions))]
pub const CRAB_SCORES: &[u64] = &[4, 40, 404, 405, 406, 407, 408];

//
// Game Plugin
//
pub(crate) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.register_type::<BugPathWalk>()
            .register_type::<BugData>()
            .add_event::<BugEntityClickedEvent>();
        #[cfg(feature = "inspect")]
        app.register_type::<ScoreTextResource>();
        app.insert_resource(ScoreTextResource(0))
            .add_systems(
                Startup,
                (
                    |mut cmd: Commands,
                     asset_server: Res<AssetServer>,
                     mut meshes: ResMut<Assets<Mesh>>,
                     mut materials: ResMut<Assets<StandardMaterial>>| {
                        cmd.insert_resource(BugsSpawnTimer {
                            timer: Timer::from_seconds(2., TimerMode::Once),
                            click_audio: asset_server.load("audio/cursor.wav"),
                            cube: meshes.add(shape::Box::new(2., 0., 3.).into()),
                            material: materials.add(Color::ORANGE.with_a(0.).into()),
                            models: vec![
                                asset_server.load("animated_3d/spider.glb#Scene0"), //spider
                                asset_server.load("animated_3d/crab.glb#Scene0"),   //crab
                            ],
                            animations: vec![
                                // Spider Walk
                                asset_server.load("animated_3d/spider.glb#Animation4"),
                                // Spider Death
                                asset_server.load("animated_3d/spider.glb#Animation1"),
                                // Crab Walk
                                asset_server.load("animated_3d/crab.glb#Animation1"),
                                // Crab Death
                                asset_server.load("animated_3d/crab.glb#Animation0"),
                            ],
                        })
                    },
                ),
            )
            .add_systems(Update, start_game)
            .add_systems(
                Update,
                (
                    factory_bugs,
                    score_print,
                    animate_bugs,
                    movement_bugs,
                    kill_detect,
                )
                    .run_if(in_state(GameState::Game)),
            );
    }
}

//
// Score Data Management
//
#[derive(Resource)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
pub(crate) struct ScoreTextResource(pub u64);

#[derive(Resource)]
struct BugsSpawnTimer {
    timer: Timer,
    click_audio: Handle<AudioSource>,
    cube: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    models: Vec<Handle<Scene>>,
    animations: Vec<Handle<AnimationClip>>,
}

#[derive(Clone, Default)]
struct BugAnimations {
    walk: Handle<AnimationClip>,
    death: Handle<AnimationClip>,
}

impl BugAnimations {
    pub fn factory(score: u64, asset: &ResMut<BugsSpawnTimer>) -> (Handle<Scene>, Self) {
        let (model, anims) = if CRAB_SCORES.contains(&score) {
            (asset.models[1].clone_weak(), &asset.animations[2..=3])
        } else {
            (asset.models[0].clone_weak(), &asset.animations[0..=1])
        };
        (
            model,
            Self {
                walk: anims[0].clone_weak(),
                death: anims[1].clone_weak(),
            },
        )
    }
}

#[derive(Event)]
struct BugEntityClickedEvent(Entity, Option<Vec3>);

impl From<ListenerInput<Pointer<Down>>> for BugEntityClickedEvent {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        Self(event.target, event.hit.position)
    }
}
