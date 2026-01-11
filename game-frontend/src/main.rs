#![allow(clippy::must_use_candidate)]

#[allow(clippy::pedantic, clippy::nursery)]
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

include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));

fn main() {
    use app::App;

    leptos::mount::mount_to_body(App);
}
