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
pub mod cell;
pub mod components;
pub mod game;

// use leptos::*;
//

fn main() {
    console_error_panic_hook::set_once();
    let mut game = Game::new(4);
    game.board.squares[0][1].revive();
    game.print_to_terminal();
    game = game.next_generation();
    println!("------");
    let mut i = 0;

    loop {
        game = game.next_generation();

        game.print_to_terminal();
        i += 1;
        if i == 10 {
            break;
        }
        println!("------");
    }
    mount_to_body(|| view! { <App/> });
}
//trunk serve --open
