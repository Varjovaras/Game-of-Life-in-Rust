use leptos::{component, create_signal, view, CollectView, IntoView, SignalGet, SignalUpdate};

use crate::game::Game;

#[component]
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn App() -> impl IntoView {
    let (game_signal, set_game_signal) = create_signal(Game::default());
    // let (square_signal, set_square_signal) = create_signal(game_signal.get().board.squares);
    let (count, set_count) = create_signal(0);
    let (game_over, set_game_over) = create_signal("");

    view! {
        <div class="text-center">

        {game_over}
            {move || {
                game_signal
                    .get()
                    .board
                    .squares
                    .into_iter()
                    .map(|row| {
                        view! {
                            <ul>
                                <li>
                                    {row
                                        .into_iter()
                                        .map(|square| {
                                            view! { <span>{square.current_status()} "   "</span> }
                                        })
                                        .collect_view()}
                                </li>
                            </ul>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
            <button
                class="border border-t-4"
                on:click=move |_| {
                    set_count(3);
                    set_game_signal
                        .update(move |game| {
                            match game.next_generation() {
                                Some(new_game) => *game = new_game,
                                None => {
                                    set_game_over("game over");
                                }
                            }
                        });
                }
            >

                {"Next Generation"}
            </button> <div>{move || game_signal.get().generation}</div>
        </div>
    }
}
