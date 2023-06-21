use std::time::Duration;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::TweenCompleted;
use rand::{thread_rng, Rng};

use crate::{
    effects::EffectTypeEvent, ext::Vec3ExtMut, helper::generate_points, GameState,
    MAX_BUGS_ON_SCREEN,
};

use super::{
    components::*, BugAnimations, BugEntityClickedEvent, BugsSpawnTimer, ScoreTextResource,
};

//
// Generation of bug entities
//
pub(super) fn factory_bugs(
    mut cmd: Commands,
    time: Res<Time>,
    mut spawn_data: ResMut<BugsSpawnTimer>,
    bugs: Query<Entity, With<BugData>>,
    score: Res<ScoreTextResource>,
) {
    if !spawn_data.timer.tick(time.delta()).finished() || bugs.iter().count() >= *MAX_BUGS_ON_SCREEN
    {
        return;
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
        BugData::factory(score.0, animations),
        BugPathWalk {
            points,
            current_path: 0,
            speed: 2.5,
        },
    ))
    .with_children(|parent| {
        parent.spawn(SceneBundle {
            scene,
            transform: Transform::from_translation(Vec3::new(0., -1., 0.5)),
            ..default()
        });
    });

    // Change timer and reset
    spawn_data
        .timer
        .set_duration(Duration::from_secs(rnd.gen_range(2u64..=5u64)));
    spawn_data.timer.reset();
}

//
// Movement of bug entity on screen
//
pub(super) fn movement_bugs(
    mut cmd: Commands,
    time: Res<Time>,
    mut bugs: Query<(Entity, &mut BugData, &mut Transform, &mut BugPathWalk)>,
) {
    for (entity, mut data, mut transform, mut path) in bugs.iter_mut() {
        if data.is_dead() {
            continue;
        }
        if data.state == BugState::Idle {
            data.state = BugState::Walking;
        }
        if let Some(next) = path.points.get(path.current_path + 1) {
            transform.look_at(path.points[path.current_path], Vec3::Z);
            if transform.move_towards(next, path.speed * time.delta_seconds()) {
                path.current_path += 1;
            }
        } else {
            cmd.entity(entity).despawn_recursive();
        }
    }
}

pub(super) fn animate_bugs(
    mut animation_player: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
    mut bugs: Query<(Entity, &mut BugData)>,
) {
    for (entity, mut data) in bugs.iter_mut() {
        if data.state == BugState::Idle || data.state == data.last_state {
            continue;
        }
        data.last_state = data.state;
        for children_entity in children.iter_descendants(entity) {
            if let Ok(mut anim) = animation_player.get_mut(children_entity) {
                match data.state {
                    BugState::Walking => {
                        anim.play(data.animations.walk.clone_weak())
                            .set_speed(1.5)
                            .repeat();
                    }
                    BugState::Death => {
                        anim.stop_repeating()
                            .play_with_transition(
                                data.animations.death.clone_weak(),
                                Duration::from_millis(250),
                            )
                            .set_speed(1.0);
                    }
                    _ => {}
                }
            }
        }
    }
}

//
// Kill Bug Entity when is clicked or end walk
//
pub(super) fn kill_detect(
    mut cmd: Commands,
    time: Res<Time>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    spawn_data: Res<BugsSpawnTimer>,
    mut bugs: Query<(Entity, &Transform, &mut BugData), With<BugPathWalk>>,
    mut score: ResMut<ScoreTextResource>,
    mut click_event: EventReader<BugEntityClickedEvent>,
    mut effect: EventWriter<EffectTypeEvent>,
) {
    let clicks = click_event.iter().collect::<Vec<&BugEntityClickedEvent>>();

    for (entity, bug_transform, mut data) in bugs.iter_mut() {
        // if bug is killed
        if data.is_dead() {
            let mut entity = cmd.entity(entity);
            // play dead animation
            if data.state != BugState::Death {
                score.0 += 1;
                data.state = BugState::Death;
                entity
                    .remove::<PickableBundle>()
                    .remove::<RaycastPickTarget>()
                    .remove::<OnPointer<Click>>();
            }
            // run countdown for remove from scene
            if data.wait_for_remove.tick(time.delta()).finished() {
                effect.send(EffectTypeEvent::Dead {
                    pos: bug_transform.translation,
                });
                entity.despawn_recursive();
            }
            continue;
        }
        for e in &clicks {
            // if not clicked same entity as iter
            if e.0.index() != entity.index() {
                continue;
            }
            data.clicks += 1;
            if let Some(sink) = audio_sinks.get(&audio.play(spawn_data.click_audio.clone())) {
                sink.set_volume(0.5);
            }
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

//
// Print on screen the score
//
pub(super) fn score_print(
    mut text: Query<&mut Text, With<ScoreText>>,
    mut anim_reader: EventReader<TweenCompleted>,
    mut game_state: ResMut<NextState<GameState>>,
    score: Res<ScoreTextResource>,
) {
    if text.is_empty() {
        return;
    }
    let mut text = text.single_mut();
    text.sections[0].value = score.0.to_string();

    for e in anim_reader.iter() {
        if e.user_data == 2 {
            game_state.set(GameState::Game);
        }
    }
}
