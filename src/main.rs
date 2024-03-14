#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::empty_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]

use app::App;
use game::Game;

use leptos::{mount_to_body, view};

pub mod app;
pub mod board;
pub mod components;
pub mod game;
pub mod square;

// use leptos::*;
//

fn main() {
    console_error_panic_hook::set_once();
    let mut game = Game::default();
    game.print_to_terminal();
    game = game.next_generation();
    println!("------");
    game.print_to_terminal();
    mount_to_body(|| view! { <App/> });
}
//trunk serve --open
