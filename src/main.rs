#![allow(dead_code)]
#![allow(unused_variables)]

// use leptos::*;

#[derive(Debug, Clone)]
enum Status {
    Alive,
    Dead,
}

impl Status {}

#[derive(Debug, Clone)]
struct Square(Status);

impl Square {
    const fn new() -> Self {
        Self(Status::Dead)
    }
}

#[derive(Debug)]
struct Board {
    squares: Vec<Vec<Square>>,
}

impl Board {
    fn new() -> Self {
        let squares = vec![vec![Square::new(); 4]];
        Self { squares }
    }

    // fn generation(&self) {}
}

//node dies at underpopulation or 1
//node dies at overpopulation or more
//node lives at between underpopulation and underpopulation
//node resurrects at reproduction
struct Rules {
    underpopulation: u8,
    overpopulation: u8,
    reproduction: u8,
}

impl Rules {
    const fn new() -> Self {
        Self {
            underpopulation: 1,
            overpopulation: 4,
            reproduction: 3,
        }
    }
}

struct Game {
    board: Board,
}

fn main() {
    // mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
