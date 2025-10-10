#![allow(clippy::must_use_candidate)]

#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
pub mod common {
    tonic::include_proto!("common");
}

#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
pub mod game {
    tonic::include_proto!("game");
}

pub mod app;
pub mod views;

use crate::app::App;

fn main() {
    leptos::mount::mount_to_body(App);
}
