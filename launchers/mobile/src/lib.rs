use bevy::prelude::*;

fn open_url(_url: &str) {}

#[bevy_main]
fn main() {
    println!("Starting launcher: Mobile");
    game::app(true, game::LocaleLangs::EN, open_url).run();
}
