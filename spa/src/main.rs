use app::app::App;
use leptos::{logging, mount_to_body};

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn wasm_main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("spa mode - mounting to body");

    mount_to_body(App);
}

pub fn main() {}
