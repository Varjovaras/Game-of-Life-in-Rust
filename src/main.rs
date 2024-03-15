#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::empty_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]

use app::App;

use leptos::{mount_to_body, view};

pub mod app;
pub mod board;
pub mod cell;
pub mod components;
pub mod game;
pub mod status;
pub mod terminal;

// use leptos::*;
//

fn main() {
    console_error_panic_hook::set_once();
    terminal::run();
    mount_to_body(|| view! { <App/> });
}
//trunk serve --open
