use std::time::Duration;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::TweenCompleted;
use rand::prelude::*;

use crate::{
    effects::EffectTypeEvent, ext::Vec3ExtMut, helper::generate_points, GameState,
    MAX_BUGS_ON_SCREEN,
};

//
// Score Data Management
//
#[derive(Resource)]
pub(crate) struct ScoreTextResource(pub u64);

//
// Score Text Identifier
//
#[derive(Component)]
pub(crate) struct ScoreText;

//
// Patrol Data for bugs
//
#[derive(Component, Clone, Reflect)]
#[reflect]
struct BugPathWalk {
    current_path: usize,
    points: Vec<Vec3>,
    speed: f32,
}

//
// Bug Data
//
#[derive(Component, Reflect)]
#[reflect]
struct BugData {
    clicks: u8,
    max_clicks: u8,
    wait_for_remove: Timer, // when is dead, this tick for despawn entity
    animations: BugAnimations,
}

impl BugData {
    pub fn is_dead(&self) -> bool {
        self.clicks >= self.max_clicks
    }

    pub fn factory(score: u64, animations: BugAnimations) -> Self {
        #[cfg(debug_assertions)]
        let max_clicks = if score == 10 { 2 } else { 1 };
        #[cfg(not(debug_assertions))]
        let max_clicks = if score == 404 { 2 } else { 1 };
        Self {
            clicks: 0,
            max_clicks,
            animations,
            wait_for_remove: Timer::from_seconds(3., TimerMode::Once),
        }
    }
}

#[derive(Resource)]
struct BugsSpawnTimer {
    timer: Timer,
    cube: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    models: Vec<Handle<Scene>>,
    animations: Vec<Handle<AnimationClip>>,
}

#[derive(Resource, Clone, Reflect)]
#[reflect]
struct BugAnimations {
    walk: Handle<AnimationClip>,
    death: Handle<AnimationClip>,
}

impl BugAnimations {
    pub fn factory(score: u64, asset: &ResMut<BugsSpawnTimer>) -> (Handle<Scene>, Self) {
        let (model, anims) = if score == 404 {
            (asset.models[1].clone_weak(), &asset.animations[0..1])
        } else {
            (asset.models[0].clone_weak(), &asset.animations[2..3])
        };
        (
            model,
            Self {
                walk: anims[0],
                death: anims[1],
            },
        )
    }
}

// #[derive(Resource)]
// struct BugsSpawnTimer(Timer);

struct BugEntityClickedEvent(Entity, Option<Vec3>);

impl From<ListenedEvent<Click>> for BugEntityClickedEvent {
    fn from(event: ListenedEvent<Click>) -> Self {
        BugEntityClickedEvent(event.target, event.hit.position)
    }
}

//
// Game Plugin
//
pub(crate) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScoreTextResource(0))
            .register_type::<BugPathWalk>()
            .register_type::<BugData>()
            .add_event::<BugEntityClickedEvent>()
            .add_startup_system(
                |mut cmd: Commands,
                 mut data: ResMut<BugsSpawnTimer>,
                 asset_server: Res<AssetServer>,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<StandardMaterial>>| {
                    cmd.insert_resource(BugsSpawnTimer {
                        timer: Timer::from_seconds(2., TimerMode::Once),
                        cube: meshes.add(shape::Box::new(2., 0., 3.).into()),
                        material: materials.add(Color::WHITE.with_a(0.).into()),
                        models: vec![
                            asset_server.load("animated_3d/spider.glb#Scene0"), //spider
                            asset_server.load("animated_3d/spider.glb#Scene0"), //crab
                        ],
                        animations: vec![
                            asset_server.load("animated_3d/spider.glb#Animation4"), // Spider Walk
                            asset_server.load("animated_3d/spider.glb#Animation1"), // Spider Death
                            asset_server.load("animated_3d/spider.glb#Animation4"), // Crab Walk
                            asset_server.load("animated_3d/spider.glb#Animation1"), // Crab Death
                        ],
                    })
                },
            )
            .add_systems((
                factory_bugs
                    .pipe(setup_animation)
                    .run_if(in_state(GameState::Game)),
                movement_bugs,
                kill_detect,
                score_print,
            ));
    }
}

fn factory_bugs(
    mut cmd: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut spawn_data: ResMut<BugsSpawnTimer>,
    bugs: Query<Entity, With<BugData>>,
    score: Res<ScoreTextResource>,
) -> Result<BugAnimations, ()> {
    if !spawn_data.timer.tick(time.delta()).finished() || bugs.iter().count() >= *MAX_BUGS_ON_SCREEN
    {
        return Err(());
    }
    let mut rnd = thread_rng();
    let points = generate_points(rnd.clone());
    let (scene, animations) = BugAnimations::factory(score.0, &spawn_data);
    // Spawning a cube to experiment on
    cmd.spawn((
        PbrBundle {
            mesh: spawn_data.cube.clone(),
            material: spawn_data.material.clone(),
            transform: Transform::from_translation(points[0]).with_scale(Vec3::splat(1.)),
            ..default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
        OnPointer::<Click>::send_event::<BugEntityClickedEvent>(),
        BugData::factory(score.0, animations.clone()),
        BugPathWalk {
            points,
            current_path: 0,
            speed: 2.5,
        },
    ))
    .with_children(|parent| {
        parent.spawn(SceneBundle {
            scene,
            transform: Transform::from_translation(Vec3::new(0., -1., 1.)),
            ..default()
        });
    });

    // Change timer and reset
    spawn_data
        .timer
        .set_duration(Duration::from_secs(rnd.gen_range(2u64..=5u64)));
    spawn_data.timer.reset();
    Ok(animations)
}

fn setup_animation(
    In(animations): In<Result<BugAnimations, ()>>,
    mut bug_animation: Query<&mut AnimationPlayer>,
) {
    if let Ok(animations) = animations {
        for mut bug in bug_animation.iter_mut() {
            bug.play(animations.walk.clone_weak()).repeat();
        }
    }
}

fn movement_bugs(
    mut cmd: Commands,
    time: Res<Time>,
    mut bugs: Query<(Entity, &BugData, &mut Transform, &mut BugPathWalk)>,
) {
    for (entity, data, mut transform, mut path) in bugs.iter_mut() {
        if data.is_dead() {
            continue;
        }
        if let Some(next) = path.points.get(path.current_path + 1) {
            transform.look_at(*next, Vec3::Z);
            if transform.move_towards(next, path.speed * time.delta_seconds()) {
                path.current_path += 1;
            }
        } else {
            cmd.entity(entity).despawn_recursive();
        }
    }
}

fn kill_detect(
    mut cmd: Commands,
    time: Res<Time>,
    mut animation_player: Query<&mut AnimationPlayer>,
    mut bugs: Query<(Entity, &Transform, &mut BugData), With<BugPathWalk>>,
    mut score: ResMut<ScoreTextResource>,
    mut click_event: EventReader<BugEntityClickedEvent>,
    mut effect: EventWriter<EffectTypeEvent>,
) {
    let clicks = click_event
        .iter()
        .map(|e| e)
        .collect::<Vec<&BugEntityClickedEvent>>();

    for (entity, bug_transform, mut data) in bugs.iter_mut() {
        // if bug is killed
        if data.is_dead() {
            // play dead animation
            if let Ok(mut bug) = animation_player.get_single_mut() {
                println!("Play Death");
                bug.play_with_transition(
                    data.animations.death.clone_weak(),
                    Duration::from_millis(250),
                );
            }
            // run countdown for remove from scene
            if data.wait_for_remove.tick(time.delta()).finished() {
                effect.send(EffectTypeEvent::Dead {
                    pos: bug_transform.translation,
                });
                cmd.entity(entity).despawn_recursive();
            }
            continue;
        }
        for e in &clicks {
            // if not clicked same entity as iter
            if e.0.index() != entity.index() {
                continue;
            }
            score.0 += 1;
            data.clicks += 1;
            // Spawn particles
            let pos = if let Some(p) = e.1 {
                p
            } else {
                bug_transform.translation
            };
            effect.send(EffectTypeEvent::Click { pos });
        }
    }
}

fn score_print(
    mut text: Query<&mut Text, With<ScoreText>>,
    mut anim_reader: EventReader<TweenCompleted>,
    mut game_state: ResMut<NextState<GameState>>,
    score: Res<ScoreTextResource>,
) {
    let mut text = text.single_mut();
    text.sections[0].value = score.0.to_string();

    for e in anim_reader.iter() {
        if e.user_data == 2 {
            game_state.set(GameState::Game);
        }
    }
}
