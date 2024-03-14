use leptos::{component, create_signal, view, CollectView, IntoView, SignalGet, SignalUpdate};

use crate::game::Game;

#[component]
#[must_use]
#[allow(clippy::module_name_repetitions)]
pub fn App() -> impl IntoView {
    let (game_signal, set_game_signal) = create_signal(Game::default());
    // let (square_signal, set_square_signal) = create_signal(game_signal.get().board.squares);
    let (count, set_count) = create_signal(0);

    let squares = game_signal
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
        .collect::<Vec<_>>();

    view! {
        <div class="text-center">

            {squares}
            {game_signal
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
                .collect::<Vec<_>>()}
            <button
                class="border border-t-4"
                on:click=move |_| {
                    set_count(3);
                    set_game_signal
                        .update(move |game| {
                            *game = game.next_generation();
                        });
                }
            >

                {"Next Generation"}
            </button> <div>{count}</div>
        </div>
    }
}
