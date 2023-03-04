use macroquad::prelude::*;
use std::{thread, time};

pub struct Vector2i {
    x: i32,
    y: i32
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Vector2i {
        return Vector2i { x: x, y: y };
    }

    /**
     * Add two vectors and store in this object.
     */
    pub fn add(&self, a: Vector2i) -> Vector2i {
        return Vector2i::new(self.x + a.x, self.y + a.y);
    }
}

pub struct TetrisBoard {
    size: Vector2i,
    block: Vector2i,
    board: [[i32; 10]; 20],
}


pub struct Tetris {
    running: bool,

    // Store the current game score
    score: i32,
}

impl Tetris {

    pub fn reset() {

    }

    pub fn change_state(state: i32) {

    }

    pub fn update() {

    }
    
    pub fn draw() {
            
    
    }
}




#[macroquad::main("Retris")]
async fn main() {

    let running = true;

    // Store the current game score
    let mut score = 0;

    // Board size
    let size: Vector2i = Vector2i::new(10, 20);

    let mut pieces: [[[i32; 4]; 3]; 7] = [
        [
            [1, 1, 1, 1],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ],
        [
            [1, 0, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0]
        ],
        [
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ],
        [
            [0, 0, 0, 1],
            [0, 1, 1, 1],
            [0, 0, 0, 0]
        ],
        [
            [0, 1, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0]
        ],
        [
            [0, 0, 1, 1],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ],
        [
            [1, 1, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ]
    ];

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
        [0, 0, 0, 0, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 4, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 2, 0, 0, 0, 0],
        [0, 0, 0, 2, 3, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 2, 3, 4, 5, 6, 7, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    ];

    let mut colors: [macroquad::color::Color; 8] = [
        macroquad::color::BLACK,
        macroquad::color::GREEN,
        macroquad::color::BLUE,
        macroquad::color::RED,
        macroquad::color::ORANGE,
        macroquad::color::PURPLE,
        macroquad::color::YELLOW,
        macroquad::color::LIME
    ];

    
    while running {
        clear_background(BLACK);

        let width: f32 = screen_width();
        let height: f32 = screen_height();
        
        let block: f32 = height / size.y as f32;
        let origin: f32 = width as f32 / 2.0 - (size.x as f32 * block) / 2.0;

        let mut x: i32 = 0;
        while x < size.x {
            let mut y: i32 = 0;
            while y < size.y {
                draw_rectangle(origin + block * x as f32, block * y as f32, block as f32, block, colors[board[y as usize][x as usize] as usize]);
                draw_rectangle_lines(origin + block * x as f32, block * y as f32, block as f32, block, 2.0, macroquad::color::GRAY);
                y += 1;
            }
            x += 1;
        }

        // let ms: time::Duration = time::Duration::from_millis(1000);
        // thread::sleep(ms);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(width / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(width - 30.0, height - 30.0, 15.0, YELLOW);
        
        draw_text(&(String::from("Score: ") + &score.to_string()), 10.0, 20.0, 30.0, macroquad::color::WHITE);

        if is_key_down(KeyCode::A) {
            println!("A is down");
        }

        score += 1;

        // Close application
        if is_key_down(KeyCode::Escape) {
            return;
        }

        next_frame().await;
    }

    println!("Hello, world!");
}
