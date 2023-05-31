use bevy::prelude::*;

fn open_url(url: &str) {
    open::that(url).unwrap();
}

fn main() {
    info!("Starting launcher: Native");
    game::app(true, game::LocaleLangs::EN, open_url).run();
}
