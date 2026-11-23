mod app;
mod engine;
mod templates;
mod types;
mod views;

use app::App;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
