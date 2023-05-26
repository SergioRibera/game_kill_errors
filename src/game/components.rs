use bevy::prelude::*;

use super::{BugAnimations, CRAB_SCORES};

//
// Score Text Identifier
//
#[derive(Component)]
pub struct ScoreText;

//
// Patrol Data for bugs
//
#[derive(Component, Clone, Reflect)]
#[reflect]
pub(super) struct BugPathWalk {
    pub(super) current_path: usize,
    pub(super) points: Vec<Vec3>,
    pub(super) speed: f32,
}

#[derive(Clone, Copy, Default, Reflect, PartialEq, Eq)]
#[reflect]
pub(super) enum BugState {
    #[default]
    Idle,
    Walking,
    Death
}

//
// Bug Data
//
#[derive(Component, Reflect)]
#[reflect]
pub(super) struct BugData {
    pub(super) clicks: u8,
    pub(super) max_clicks: u8,
    pub(super) wait_for_remove: Timer, // when is dead, this tick for despawn entity
    pub(super) last_state: BugState,
    pub(super) state: BugState,
    #[reflect(ignore)]
    pub(super) animations: BugAnimations,
}

impl BugData {
    pub fn is_dead(&self) -> bool {
        self.clicks >= self.max_clicks
    }

    pub fn factory(score: u64, animations: BugAnimations) -> Self {
        let max_clicks = if CRAB_SCORES.contains(&score) { 2 } else { 1 };
        Self {
            clicks: 0,
            max_clicks,
            animations,
            state: BugState::Idle,
            last_state: BugState::Idle,
            wait_for_remove: Timer::from_seconds(3., TimerMode::Once),
        }
    }
}
