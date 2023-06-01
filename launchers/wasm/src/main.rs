use game::{LocaleLangs, LAUNCHER_TITLE};
use yew::prelude::*;

fn open_url(url: &str) {
    let window = web_sys::window().unwrap();
    window.location().replace(url).unwrap();
}

fn set_window_title(title: &str) {
    web_sys::window()
        .and_then(|w| w.document())
        .expect("Unable to get DOM")
        .set_title(title);
}

fn get_lang() -> LocaleLangs {
    if let Some(lang) = web_sys::window()
        .and_then(|w| w.document())
        .expect("Unable to get DOM")
        .location()
        .expect("Unable to get Location")
        .hash()
        .ok()
        .and_then(|v| if v.is_empty() { None } else { Some(v) })
        .map(|h| {
            log::info!("The lang from hash is {h}");
            if h.to_lowercase().contains("es") {
                LocaleLangs::ES
            } else {
                LocaleLangs::EN
            }
        })
    {
        log::info!("From Hash: {lang:?}");
        return lang;
    }
    let nav_lang = web_sys::window()
        .map(|w| w.navigator())
        .expect("Unable to get navigator")
        .language()
        .unwrap();
    log::info!("The lang is {nav_lang}");
    if nav_lang.to_lowercase().starts_with("es") {
        return LocaleLangs::ES;
    }
    LocaleLangs::EN
}

#[function_component(Root)]
fn view() -> Html {
    set_window_title(LAUNCHER_TITLE);

    html! {
        <> </>
    }
}

fn main() {
    #[cfg(feature = "inspect")]
    wasm_logger::init(
        wasm_logger::Config::new(log::Level::Info), // .module_prefix("wasm_kill_errors")
                                                    // .module_prefix("game"),
    );
    // Mount the DOM
    yew::Renderer::<Root>::new().render();
    // Start the Bevy App
    log::info!("Starting launcher: WASM");
    game::app(false, get_lang(), open_url).run();
}
