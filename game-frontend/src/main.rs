#![allow(clippy::must_use_candidate)]

#[allow(clippy::pedantic)]
#[allow(clippy::nursery)]
pub mod pb {
    pub mod common {
        pub mod v1 {
            tonic::include_proto!("common.v1");
        }
    }
    pub mod game {
        pub mod v1 {
            tonic::include_proto!("game.v1");
        }
    }
}

pub mod app;
pub mod views;

use app::App;

fn main() {
    leptos::mount::mount_to_body(App);
}
