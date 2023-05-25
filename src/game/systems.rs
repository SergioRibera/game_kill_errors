use std::time::Duration;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::TweenCompleted;
use rand::{thread_rng, Rng};

use crate::{
    effects::EffectTypeEvent, ext::Vec3ExtMut, helper::generate_points, GameState, MAX_BUGS_ON_SCREEN,
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

//
// Pipe from factory of entity
// Enable first animation (walk)
//
pub(super) fn setup_animation(
    In(animations): In<Result<BugAnimations, ()>>,
    mut bug_animation: Query<&mut AnimationPlayer>,
) {
    if let Ok(animations) = animations {
        for mut bug in bug_animation.iter_mut() {
            bug.play(animations.walk.clone_weak()).repeat();
        }
    }
}

//
// Movement of bug entity on screen
//
pub(super) fn movement_bugs(
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

//
// Kill Bug Entity when is clicked or end walk
//
pub(super) fn kill_detect(
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

//
// Print on screen the score
//
pub(super) fn score_print(
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
