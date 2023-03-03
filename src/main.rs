use macroquad::prelude::*;
use std::ops::Add;

pub struct Vector2i {
    x: i32,
    y: i32
}

// impl Vector2i {
//     /**
//      * Add two vectors and store in this object.
//      */
//     fn add(&self, a: Vector2i) -> () {
//         self.x += a.x;
//         self.y += a.y;
//     }
// }

#[macroquad::main("Retris")]
async fn main() {

    let running = true;

    // Store the current game score
    let mut score = 0;

    // Board size
    let board_size: Vector2i = Vector2i{x: 10, y: 20};

    // Current board status
    let mut board: [[i32; 10]; 20] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    ];

    
    while running {
        clear_background(BLACK);

        let width: f32 = screen_width();
        let height: f32 = screen_height();
        
        


        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(width / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(width - 30.0, height - 30.0, 15.0, YELLOW);
        
        let mut x: i32 = 0;
        while x < board_size.x {
            let mut y: i32 = 0;
            while y < board_size.y {
                draw_rectangle(30.0 * x as f32, 30.0 * y as f32, 30.0, 30.0, GREEN);
                y += 1;
            }
            x += 1;
        }


        if is_key_down(KeyCode::A) {
            println!("A is down");
        }

        // Close application
        if is_key_down(KeyCode::Escape) {
            return;
        }

        next_frame().await;
    }

    println!("Hello, world!");
}
