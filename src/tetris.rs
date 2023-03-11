use crate::board::TetrisBoard;
use crate::piece::Piece;
use crate::vector2::Vector2i;

pub struct Tetris {
    // Board
    pub board: TetrisBoard,
    
    // Currently moving piece.
    pub piece: Piece,

    // Flag to indicate if the game is running
    pub running: bool,

    // Pause control flag
    pub paused: bool,

    // Store the current game score
    pub score: i32,

    // Count how many frames ellapsed since the beginning of the game
    pub frame: i32,
}

impl Tetris {

    pub fn new() -> Tetris {
        return Tetris{
            board: TetrisBoard::new(),
            piece: Piece::new(),
            running: true,
            paused: false,
            frame: 0,
            score: 0
        }
    }

    pub fn reset(mut self) {

    }

    // Check if there are any full rows.
    //
    // Destroy the rows and increase score.
    pub fn check_rows(&mut self) {
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
    pub fn piece_can_move(&mut self, delta: Vector2i) -> bool {
        let mut y: i32 = 0;
        while y < self.piece.piece.len() as i32 {
            let mut x: i32 = 0;
            while x < self.piece.piece[y as usize].len() as i32 {
                if self.piece.piece[y as usize][x as usize] != 0 {
                    // Check if piece in range inside of the tetris board
                    if self.piece.pos.x + x + delta.x < 0 || self.piece.pos.x + x + delta.x >= self.board.size.x {
                        return false
                    }

                    if self.piece.pos.y + y + delta.y < 0 || self.piece.pos.y + y + delta.y >= self.board.size.y {
                        return false
                    }
                    
                    // Check if will collide with an existing piece on the tetris.board
                    if self.board.board[(self.piece.pos.y + y + delta.y) as usize][(self.piece.pos.x + x + delta.x) as usize] != 0 {
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

    // Change the state of the tetris game
    pub fn change_state(&mut self, running: bool) {
        self.running = running;
    }

    pub fn update() {

    }
    
    pub fn draw() { 
    
    }
}

