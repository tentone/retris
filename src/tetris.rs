use macroquad::prelude::*;

use crate::board::TetrisBoard;
use crate::color::colors;
use crate::piece::Piece;
use crate::vector2::Vector2i;

pub enum TetrisState {
    Running,
    GameOver,
    Exit
}

pub struct Tetris {
    // Board
    pub board: TetrisBoard,
    
    // Currently moving piece.
    pub piece: Piece,

    // Flag to indicate if the game is running
    pub state: TetrisState,

    // Pause control flag
    pub paused: bool,

    // Store the current game score
    pub score: i32,

    // Count how many frames ellapsed since the beginning of the game
    pub frame: i32,
}

impl Tetris {
    // Create a new tetris game object
    pub fn new() -> Tetris {
        return Tetris{
            board: TetrisBoard::new(),
            piece: Piece::new(),
            state: TetrisState::Running,
            paused: false,
            frame: 0,
            score: 0
        }
    }

    pub fn reset(&mut self) {
        self.board = TetrisBoard::new();
        self.piece = Piece::new();
        self.state = TetrisState::Running;
        self.paused = false;
        self.frame = 0;
        self.score = 0;
    }

    // Change the state of the tetris game
    pub fn setState(&mut self, state: TetrisState) {
        self.state = state;
    }

    pub async fn run(&mut self) {
        while !matches!(self.state, TetrisState::Exit) {
            self.update();
            self.render();

            self.frame += 1;

            // Close application
            if is_key_down(KeyCode::Escape) {
                self.setState(TetrisState::Exit);
            }

            next_frame().await;
        }
    }

    pub fn update(&mut self) {
        if matches!(self.state, TetrisState::GameOver) {
            if is_key_pressed(KeyCode::R) {
                self.reset();
            }
        }
        else if matches!(self.state, TetrisState::Running) {
            // Pause
            if is_key_pressed(KeyCode::P) {
                self.paused = !self.paused;
            }

            if self.paused {
                return;
            }

            // Rotate Piece
            if is_key_pressed(KeyCode::Up) {
                let mut piece = self.piece.clone();
                piece.rotate();

                if self.piece_can_move(piece, Vector2i{x: 0, y: 0}) {
                    self.piece.rotate();
                }
            }

            // Move Piece left
            if is_key_pressed(KeyCode::Left) {
                if self.piece_can_move(self.piece, Vector2i{x: -1, y: 0}) {
                    self.piece.pos.x -= 1;
                }
            }
            if is_key_pressed(KeyCode::Right) {
                if self.piece_can_move(self.piece, Vector2i{x: 1, y: 0}) {
                    self.piece.pos.x += 1;
                }
            }

            // Place the piece where it is (just for testing)
            if is_key_pressed(KeyCode::F) {
                self.place_piece();
                self.piece.random();
            }

            // Move Piece faster
            let mut speedup: bool = false;
            if is_key_down(KeyCode::Down) {
                speedup = true;
            }
            
            // Check top row
            if self.check_top_row() {
                self.setState(TetrisState::GameOver)
            }

            // Move piece down
            if self.frame % 40 == 0 || speedup && self.frame % 3 == 0 {
                // Check if the piece can move down further
                let mut piece_can_move_down: bool = self.piece_can_move(self.piece, Vector2i{x: 0, y: 1});

                // Move the piece down
                if piece_can_move_down {
                    self.piece.pos.y += 1;
                // Place piece to the tetris.board.
                } else {
                    self.place_piece();
                    self.piece.random();
                }

                self.check_full_rows();
            }
        }
    }
    
    pub fn render(&mut self) { 
        clear_background(BLACK);

        let width: f32 = screen_width();
        let height: f32 = screen_height();
        
        let block: f32 = height / self.board.size.y as f32;
        let origin: f32 = width as f32 / 2.0 - (self.board.size.x as f32 * block) / 2.0;

        // Base tetris.board
        let mut x: i32 = 0;
        while x < self.board.size.x {
            let mut y: i32 = 0;
            while y < self.board.size.y {
                draw_rectangle(origin + block * x as f32, block * y as f32, block as f32, block, colors[self.board.board[y as usize][x as usize] as usize]);
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
                let color = self.piece.piece[y as usize][x as usize];
                if color != 0 {
                    draw_rectangle(origin + block * (self.piece.pos.x + x) as f32, block * (self.piece.pos.y + y) as f32, block as f32, block, colors[color as usize]);
                }
                
                y += 1;
            }
            x += 1;
        }

        // Draw text
        draw_text(&(String::from("Frame: ") + &self.frame.to_string()), 10.0, 25.0, 30.0, macroquad::color::WHITE);
        draw_text(&(String::from("Score: ") + &self.score.to_string()), 10.0, 50.0, 30.0, macroquad::color::WHITE);

        // Pause message
        if self.paused {
            draw_rectangle(0.0, 0.0, width, height, macroquad::color::Color{r: 0.0, g: 0.0, b: 0.0, a: 0.4});
            draw_text("Paused", width / 2.0, height / 2.0, 40.0, macroquad::color::WHITE);
        }

        if matches!(self.state, TetrisState::GameOver) {
            draw_rectangle(0.0, 0.0, width, height, macroquad::color::Color{r: 0.0, g: 0.0, b: 0.0, a: 0.4});

            // measure_text("Game Over, Press R to Restart", macroquad::text::Font::default, 40, 1.0);
            draw_text("Game Over, Press R to Restart", width / 2.0 - 230.0, height / 2.0, 40.0, macroquad::color::WHITE);
        }

    }

    // Check if there is any piece in the top row.
    pub fn check_top_row(&mut self) -> bool {
        for x in 0..self.board.board[0].len() {
            if self.board.board[0][x] != 0 {
                return true;
            }
        }

        return false;
    }

    // Check if there are any full rows.
    //
    // Destroy the rows and increase score.
    pub fn check_full_rows(&mut self) {
        // Check the status of the tetris.board
        let mut y: i32 = 0;
        while y < self.board.size.y {

            // Check if any row in the tetris.board is full
            let mut row_full: bool = true;
            let mut x: i32 = 0;
            while x < self.board.size.x {
                let value: i32 = self.board.board[y as usize][x as usize];
                if value == 0 {
                    row_full = false;
                    break;
                }
                x += 1;
            }

            // If row is full remove from tetris.board and increase score
            if row_full {
                // Move rows above this one bellow
                let mut yy: i32 = y;
                while yy > 1 {
                    let mut x: i32 = 0;
                    while x < self.board.size.x {
                        self.board.board[yy as usize][x as usize] = self.board.board[(yy - 1) as usize][x as usize];
                        x += 1;
                    }

                    yy -= 1;
                }

                // Clear first row
                let mut x: i32 = 0;
                while x < self.board.size.x {
                    self.board.board[0][x as usize] = 0;
                    x += 1;
                }

                self.score += 1;
            }
            y += 1;
        }
    }

    // Check if there is a collision when the piece is moved.
    pub fn piece_can_move(&mut self, piece: Piece, delta: Vector2i) -> bool {
        let mut y: i32 = 0;
        while y < piece.piece.len() as i32 {
            let mut x: i32 = 0;
            while x < piece.piece[y as usize].len() as i32 {
                if piece.piece[y as usize][x as usize] != 0 {
                    // Check if piece in range inside of the tetris board
                    if piece.pos.x + x + delta.x < 0 || piece.pos.x + x + delta.x >= self.board.size.x {
                        return false
                    }

                    if piece.pos.y + y + delta.y < 0 || piece.pos.y + y + delta.y >= self.board.size.y {
                        return false
                    }
                    
                    // Check if will collide with an existing piece on the tetris.board
                    if self.board.board[(piece.pos.y + y + delta.y) as usize][(piece.pos.x + x + delta.x) as usize] != 0 {
                        return false;
                    }
                }
                
                x += 1;
            }
            y += 1;
        }

        return true;
    }

    // Place piece into the tetris board.
    pub fn place_piece(&mut self) {
        let mut y: i32 = 0;

        while y < self.piece.piece.len() as i32 {
            let mut x: i32 = 0;
            while x < self.piece.piece[y as usize].len() as i32 {
                if self.piece.piece[y as usize][x as usize] != 0 {
                    self.board.board[(y + self.piece.pos.y) as usize][(x + self.piece.pos.x) as usize] = self.piece.piece[y as usize][x as usize];
                }
                
                x += 1;
            }
            y += 1;
        }
    }
}

