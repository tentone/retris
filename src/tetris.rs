
pub struct Tetris {
    pub board: TetrisBoard,
    
    // Status
    pub running: bool,

    // Store the current game score
    pub score: i32,
}

impl Tetris {

    pub fn reset() {

    }

    pub fn change_state(mut self, running: bool) {
        self.running = running;
    }

    pub fn update() {

    }
    
    pub fn draw() { 
    
    }
}

