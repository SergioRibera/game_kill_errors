use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct ScoreText(pub u64);

pub(crate) struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems((factory_bugs, kill_detect, score_print));
    }
}

fn factory_bugs() {}
fn kill_detect() {}
fn score_print() {}
