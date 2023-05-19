use bevy::prelude::*;
use yew::prelude::*;
use game::LAUNCHER_TITLE;

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
        <div class="content">
            <canvas id="bevy"></canvas>
        </div>
    }
}

fn main() {
    // Mount the DOM
    yew::Renderer::<Root>::new().render();
    // Start the Bevy App
    info!("Starting launcher: WASM");
    game::app(false, open_url).run();
}

