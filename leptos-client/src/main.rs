// use leptos::prelude::*;

pub mod app;

use crate::app::App;

fn main() {
    // leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> });
    leptos::mount::mount_to_body(App);
}
