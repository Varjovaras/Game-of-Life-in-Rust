// use leptos::*;

enum Status {
    Alive,
    Dead,
}

impl Status {}

struct Square(Status);

struct Board {
    squares: Vec<Vec<Square>>,
}

fn main() {
    // mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
