use std::time::Duration;

use bevy::{math::vec3, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_tweening::TweenCompleted;
use rand::prelude::*;

use crate::{ext::Vec3ExtMut, GameState, MAX_BUGS_ON_SCREEN};

//
// Score Data Management
//
#[derive(Component)]
pub(crate) struct ScoreText(pub u64);

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
#[derive(Component, Default, Reflect)]
#[reflect]
struct BugData {
    killed: bool,
    wait_for_remove: Timer, // when is dead, this tick for despawn entity
}

#[derive(Resource)]
struct BugsSpawnTimer {
    timer: Timer,
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
        app.insert_resource(BugsSpawnTimer {
            timer: Timer::from_seconds(2., TimerMode::Once),
        })
        .register_type::<BugPathWalk>()
        .register_type::<BugData>()
        .add_event::<BugEntityClickedEvent>()
        .add_systems((
            factory_bugs.run_if(in_state(GameState::Game)),
            movement_bugs,
            kill_detect,
            score_print,
        ));
    }
}

fn factory_bugs(
    mut cmd: Commands,
    time: Res<Time>,
    mut spawn_data: ResMut<BugsSpawnTimer>,
    bugs: Query<Entity, With<BugData>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !spawn_data.timer.tick(time.delta()).finished() || bugs.iter().count() >= *MAX_BUGS_ON_SCREEN
    {
        return;
    }
    let mut rnd = thread_rng();
    let mut points = Vec::new();

    // start point out of screen
    points.push(vec3(
        if rnd.gen_bool(0.5) {
            rnd.gen_range(-25.0..=-20.)
        } else {
            rnd.gen_range(20.0..=25.)
        },
        if rnd.gen_bool(0.5) {
            rnd.gen_range(-20.0..=-12.)
        } else {
            rnd.gen_range(12.0..=20.)
        },
        0.,
    ));

    for _ in 0..7 {
        points.push(vec3(
            rnd.gen_range(-20.0..=20.),
            rnd.gen_range(-12.0..=12.),
            0.,
        ));
    }

    // end point out of screen
    points.push(vec3(
        if rnd.gen_bool(0.5) {
            rnd.gen_range(-25.0..=-20.)
        } else {
            rnd.gen_range(20.0..=25.)
        },
        if rnd.gen_bool(0.5) {
            rnd.gen_range(-20.0..=-12.)
        } else {
            rnd.gen_range(12.0..=20.)
        },
        0.,
    ));

    // Spawning a cube to experiment on
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Cube::default().into()),
            material: materials.add(Color::ORANGE.into()),
            transform: Transform::from_translation(points[0]),
            ..default()
        },
        PickableBundle::default(),
        RaycastPickTarget::default(),
        OnPointer::<Click>::send_event::<BugEntityClickedEvent>(),
        BugData::default(),
        BugPathWalk {
            points,
            current_path: 0,
            speed: rnd.gen_range(5.0..=7.),
        },
    ));

    // Change timer and reset
    spawn_data
        .timer
        .set_duration(Duration::from_secs(rnd.gen_range(2u64..=5u64)));
    spawn_data.timer.reset();
}

fn movement_bugs(
    mut cmd: Commands,
    time: Res<Time>,
    mut bugs: Query<(Entity, &BugData, &mut Transform, &mut BugPathWalk)>,
) {
    for (entity, data, mut transform, mut path) in bugs.iter_mut() {
        if data.killed {
            continue;
        }
        if let Some(next) = path.points.get(path.current_path + 1) {
            transform.look_at(*next, Vec3::Y);
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
    mut bugs: Query<(Entity, &Transform, &mut BugData), With<BugPathWalk>>,
    mut score: Query<&mut ScoreText, With<Text>>,
    mut click_event: EventReader<BugEntityClickedEvent>,
) {
    let mut score = score.single_mut();

    for e in click_event.iter() {
        for (entity, _bug, mut data) in bugs.iter_mut() {
            // if not clicked same entity as iter
            if e.0.index() != entity.index() {
                continue;
            }
            // if bug is killed
            if data.killed {
                // TODO: add clicks to kill
                // run countdown for remove from scene
                if data.wait_for_remove.tick(time.delta()).finished() {
                    // TODO: particle desespawn
                    cmd.entity(entity).despawn_recursive();
                }
                continue;
            }
            score.0 += 1;
            data.killed = true;
            // Spawn particles
        }
    }
}

fn score_print(
    mut text: Query<(&mut Text, &ScoreText)>,
    mut anim_reader: EventReader<TweenCompleted>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (mut text, data) = text.single_mut();
    text.sections[0].value = data.0.to_string();

    for e in anim_reader.iter() {
        if e.user_data == 2 {
            game_state.set(GameState::Game);
        }
    }
}
