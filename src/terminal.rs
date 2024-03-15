use std::env;

use crate::game::Game;

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::unwrap_used)]
pub fn run() {
    let args: Vec<String> = env::args().collect();

    let squares = args
        .get(1)
        .and_then(|arg| arg.parse::<i32>().ok())
        .unwrap_or(12);

    //make query into game::new(query)
    let mut game = Game::new(squares);
    //make game into Game::new(8) if no arg found

    console_error_panic_hook::set_once();
    // let mut game = Game::new(20);
    if squares > 1 {
        game.board.squares[1][1].revive();

        game.board.squares[0][2].revive();
        game.board.squares[0][3].revive();
        game.board.squares[0][4].revive();
        game.board.squares[0][6].revive();
        game.board.squares[1][7].revive();
        game.board.squares[3][3].revive();
    }
    game.print_to_terminal();

    let mut i = 0;

    loop {
        if game.all_dead() {
            println!("All dead");
            println!("{i:?}");
            break;
        }
        if let Some(new_game) = game.next_generation() {
            game = new_game;
        } else {
            println!("No next generation");
            println!("{i:?}");
            break;
        }
        i += 1;

        if i % 100 == 0 {
            game.print_to_terminal();
            println!("------");
            println!("{i:?}");
        }
        if i == 1000 {
            game.print_to_terminal();
            println!("------");
            println!("{i:?}");
            break;
        }
    }
}
