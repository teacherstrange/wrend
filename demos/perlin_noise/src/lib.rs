use wasm_bindgen::prelude::*;

pub mod assets;
pub mod components;
pub mod graphics;
pub mod state;

/// This main function is only used for testing this portion of the app in isolation
#[wasm_bindgen]
pub fn perlin_noise() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode, so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // enables using info!() macros
    wasm_logger::init(wasm_logger::Config::default());

    // start ui
    let app_div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#app")
        .unwrap()
        .unwrap();

    yew::start_app_in_element::<components::app::App>(app_div);

    Ok(())
}
