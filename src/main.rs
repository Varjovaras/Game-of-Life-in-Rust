#![allow(dead_code)]
#![allow(unused_variables)]

use game::Game;

pub mod board;
pub mod game;
pub mod square;

// use leptos::*;
//

fn main() {
    let game = Game::default();
    // mount_to_body(|| view! { <p>"Hello, world!"</p> })
    game.print_to_terminal();
}
