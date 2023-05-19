use bevy::prelude::*;
use game::LAUNCHER_TITLE;
use yew::prelude::*;

fn open_url(url: &str) {
    let window = web_sys::window().unwrap();
    window.location().replace(url).unwrap();
}

fn set_window_title(title: &str) {
    web_sys::window()
        .map(|w| w.document())
        .flatten()
        .expect("Unable to get DOM")
        .set_title(title);
}

#[function_component(Root)]
fn view() -> Html {
    set_window_title(LAUNCHER_TITLE);

    html! {
        <> </>
    }
}

fn main() {
    // Mount the DOM
    yew::Renderer::<Root>::new().render();
    // Start the Bevy App
    info!("Starting launcher: WASM");
    game::app(false, open_url).run();
}
