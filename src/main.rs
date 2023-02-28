use macroquad::prelude::*;

pub struct Vector2<T> {
    x: T,
    y: T
}

// impl<T> Vector2<T> {
//     /**
//      * Add two vectors and store in this object.
//      */
//     fn add(&self, a: Vector2<T>) -> () {
//         self.x += a.x;
//         self.y += a.y;
//     }
// }

#[macroquad::main("Retris")]
async fn main() {

    let running = true;

    let board_size: Vector2<i32> = Vector2{x: 10, y: 20};
    // let board = [mut [mut 0u8, ..4], ..4];


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


        if (is_key_down(KeyCode::A)) {
            println!("A is down");
        }

        // Close application
        if (is_key_down(KeyCode::Escape)) {
            return;
        }

        next_frame().await;
    }

    println!("Hello, world!");
}
