use std::env;

use crate::game::Game;

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::unwrap_used)]
pub fn run() {
    let args: Vec<String> = env::args().collect();

    let squares = args
        .get(1)
        .and_then(|arg| arg.parse::<i32>().ok())
        .unwrap_or(8);

    //make query into game::new(query)
    let mut game = Game::new(squares);
    //make game into Game::new(8) if no arg found

    console_error_panic_hook::set_once();
    // let mut game = Game::new(20);
    if squares > 1 {
        game.board.squares[0][1].revive();
    }
    game.print_to_terminal();
    game = game.next_generation();
    let mut i = 0;

    loop {
        game = game.next_generation();

        i += 1;
        if i % 1000 == 0 {
            game.print_to_terminal();
            println!("------");
            println!("{i:?}");
        }
        if i == 10_000 {
            game.print_to_terminal();
            println!("------");
            println!("{i:?}");
            break;
        }
    }
}
