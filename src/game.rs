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
#[derive(Component, Default, Reflect)]
#[reflect]
struct BugData {
    clicks: u8,
    max_clicks: u8,
    wait_for_remove: Timer, // when is dead, this tick for despawn entity
}

impl BugData {
    pub fn is_dead(&self) -> bool {
        self.clicks == self.max_clicks
    }

    pub fn factory(score: u64) -> Self {
        let max_clicks = if score == 404 { 2 } else { 1 };
        Self {
            clicks: 0,
            max_clicks,
            wait_for_remove: Timer::new(Duration::from_secs(3), TimerMode::Once),
        }
    }
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
        .insert_resource(ScoreTextResource(0))
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
    score: Res<ScoreTextResource>,
) {
    if !spawn_data.timer.tick(time.delta()).finished() || bugs.iter().count() >= *MAX_BUGS_ON_SCREEN
    {
        return;
    }
    let mut rnd = thread_rng();
    let points = generate_points(rnd);
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
        BugData::factory(score.0),
        BugPathWalk {
            points,
            current_path: 0,
            speed: 2.5,
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
        if data.is_dead() {
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
    mut score: ResMut<ScoreTextResource>,
    mut click_event: EventReader<BugEntityClickedEvent>,
    mut effect: EventWriter<EffectTypeEvent>,
) {
    for e in click_event.iter() {
        for (entity, bug_transform, mut data) in bugs.iter_mut() {
            // if not clicked same entity as iter
            if e.0.index() != entity.index() {
                continue;
            }
            let Some(pos) = e.1 else { continue; };
            // if bug is killed
            if data.is_dead() {
                // TODO: add clicks to kill
                // run countdown for remove from scene
                if data.wait_for_remove.tick(time.delta()).finished() {
                    // TODO: particle desespawn
                    effect.send(EffectTypeEvent::Click {
                        pos: bug_transform.translation,
                    });
                    cmd.entity(entity).despawn_recursive();
                }
                continue;
            }
            score.0 += 1;
            data.clicks += 1;
            // Spawn particles
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
