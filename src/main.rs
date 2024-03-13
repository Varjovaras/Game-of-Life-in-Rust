#![allow(dead_code)]

// use leptos::*;
enum Status {
    Alive,
    Dead,
}

impl Status {
    #[allow(dead_code)]
    fn ali() -> i32 {
        // let ali: Option<i32> = None;
        5
    }
}

struct Square(Status);

struct Board {
    squares: Vec<Vec<Square>>,
}

fn main() {
    // mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
