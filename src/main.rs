use macroquad::prelude::*;

pub struct Vector2 {
    x: f32,
    y: f32
}

#[macroquad::main("Retris")]
async fn main() {

    let running = true;

    let board: Vector2 = Vector2{x: 10.0, y: 20.0};

    while running {
        clear_background(BLACK);

        

        next_frame().await;
    }

    println!("Hello, world!");
}
