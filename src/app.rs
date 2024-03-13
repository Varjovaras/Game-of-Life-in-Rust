use leptos::{component, create_signal, view, IntoView, SignalUpdate};

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            {count}
        </button>
    }
}
