mod app;
mod engine;
mod templates;
mod types;
mod views;

use app::App;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
