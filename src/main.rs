use macroquad::prelude::*;
use vector2::Vector2i;
use crate::tetris::Tetris;
use crate::color::colors;

mod vector2;
mod piece;
mod tetris;
mod color;
mod board;

#[macroquad::main("Retris")]
async fn main() {

    let mut tetris: Tetris = Tetris::new();

    while tetris.running {
        // ------------------- Update Logic -------------------
        // let ms = SystemTime::now().duration_since(UNIX_EPOCH).as_millis();
        // println!("{0}", ms);

        // let ms: time::Duration = time::Duration::from_millis(1000);
        // thread::sleep(ms);

        // Pause
        if is_key_pressed(KeyCode::P) {
            tetris.paused = !tetris.paused;
        }


        // Rotate Piece
        if is_key_pressed(KeyCode::Up) {
            tetris.piece.rotate();
        }

        // Move Piece left
        if is_key_pressed(KeyCode::Left) {
            if tetris.piece_can_move(Vector2i{x: -1, y: 0}) {
                tetris.piece.pos.x -= 1;
            }
        }
        if is_key_pressed(KeyCode::Right) {
            if tetris.piece_can_move(Vector2i{x: 1, y: 0}) {
                tetris.piece.pos.x += 1;
            }
        }

        // Place the piece where it is (just for testing)
        if is_key_pressed(KeyCode::F) {
            // Force-place piece in tetris.board
            tetris.place_piece();

            // Select new piece and color at random
            tetris.piece.random();
        }

        // Move Piece faster
        let mut speedup: bool = false;
        if is_key_down(KeyCode::Down) {
            speedup = true;
        }
        

        // Move piece down
        if tetris.frame % 40 == 0 || speedup && tetris.frame % 3 == 0 {
            // Check if the piece can move down further
            let mut piece_can_move_down: bool = tetris.piece_can_move(Vector2i{x: 0, y: 1});


            // Move the piece down
            if piece_can_move_down {
                tetris.piece.pos.y += 1;
            // Place piece to the tetris.board.
            } else {
                tetris.place_piece();
                tetris.piece.random();
            }

            tetris.check_rows();
        }

        // ------------------- Render code -------------------
        clear_background(BLACK);

        let width: f32 = screen_width();
        let height: f32 = screen_height();
        
        let block: f32 = height / tetris.board.size.y as f32;
        let origin: f32 = width as f32 / 2.0 - (tetris.board.size.x as f32 * block) / 2.0;

        // Base tetris.board
        let mut x: i32 = 0;
        while x < tetris.board.size.x {
            let mut y: i32 = 0;
            while y < tetris.board.size.y {
                draw_rectangle(origin + block * x as f32, block * y as f32, block as f32, block, colors[tetris.board.board[y as usize][x as usize] as usize]);
                draw_rectangle_lines(origin + block * x as f32, block * y as f32, block as f32, block, 2.0, macroquad::color::GRAY);
                y += 1;
            }
            x += 1;
        }


        // Moving piece
        let mut x: i32 = 0;
        while x < 4 {
            let mut y: i32 = 0;
            while y < 4 {
                let color = tetris.piece.piece[y as usize][x as usize];
                if color != 0 {
                    draw_rectangle(origin + block * (tetris.piece.pos.x + x) as f32, block * (tetris.piece.pos.y + y) as f32, block as f32, block, colors[color as usize]);
                }
                
                y += 1;
            }
            x += 1;
        }

        // Draw text
        draw_text(&(String::from("Frame: ") + &tetris.frame.to_string()), 10.0, 25.0, 30.0, macroquad::color::WHITE);
        draw_text(&(String::from("Score: ") + &tetris.score.to_string()), 10.0, 50.0, 30.0, macroquad::color::WHITE);

        // Pause message
        if tetris.paused {
            draw_rectangle(0.0, 0.0, width, height, macroquad::color::Color{r: 0.0, g: 0.0, b: 0.0, a: 0.4});
            draw_text("Paused", width / 2.0, height / 2.0, 40.0, macroquad::color::WHITE);
        }
        

        tetris.frame += 1;

        // Close application
        if is_key_down(KeyCode::Escape) {
            return;
        }

        next_frame().await;
    }
}
