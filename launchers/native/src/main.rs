use bevy::prelude::*;

fn open_url(url: &str) {
    open::that(url).unwrap();
}

fn main() {
    info!("Starting launcher: Native");
    game::app(true, open_url).run();
}
