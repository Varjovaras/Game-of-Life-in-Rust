#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::empty_docs)]

use app::App;
use game::Game;
pub mod app;

use leptos::{mount_to_body, view};

pub mod board;
pub mod components;
pub mod game;
pub mod square;

// use leptos::*;
//

fn main() {
    let game = Game::default();
    game.print_to_terminal();
    mount_to_body(|| view! { <App /> });
}
//trunk serve --open
