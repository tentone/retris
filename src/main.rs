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

        let width: f32 = screen_width();
        let height: f32 = screen_height();

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(width / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(width - 30.0, height - 30.0, 15.0, YELLOW);

        

        next_frame().await;
    }

    println!("Hello, world!");
}
