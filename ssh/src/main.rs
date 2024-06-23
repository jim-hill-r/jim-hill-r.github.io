#[wasm_bindgen::prelude::wasm_bindgen]
pub fn wasm_main() {
    use jimhillr_github_io::app::App;
    use leptos::{logging, mount_to_body};
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("hydrate mode - hydrating");

    mount_to_body(App);
}

pub fn main() {}
