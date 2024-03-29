use crate::{vector2::Vector2i, color, pieces};

pub struct Piece {
    // Position of the piece
    pub pos: Vector2i,

    // Size of the piece (array)
    pub size: Vector2i,

    // Piece data (with color)
    pub piece: [[i32; 4]; 4]
}

impl Clone for Piece {
    fn clone(&self) -> Piece {
        return Piece {
            pos: self.pos.clone(),
            size: self.size.clone(),
            piece: self.piece.clone()
        }
    }
}

impl Copy for Piece { }

impl Piece {
    // Create a new piece
    pub fn new() -> Piece {
        return Piece {
            pos: Vector2i::new(4, 0),
            size: Vector2i::new(4, 4),
            piece: pieces::Pieces[0].clone()
        };
    }

    // Create a new piece with random values
    pub fn newRandom() -> Piece {
        let mut piece: Piece = Piece::new();
        piece.random();
        return piece;
    }

    // Select a new piece at random with a random color
    pub fn random(&mut self) {
        let mut piece_idx: i32 = macroquad::rand::gen_range::<i32>(0, pieces::Pieces.len() as i32);
        let mut color_idx: i32 = piece_idx + 1;
        
        self.piece = pieces::Pieces[piece_idx as usize].clone();
        self.pos.set(2, 0);

        // Set color of the piece
        let mut x: i32 = 0;
        while x < self.piece.len() as i32 {
            let mut y: i32 = 0;
            while y < self.piece[x as usize].len() as i32 {
                if self.piece[x as usize][y as usize] != 0 {
                    self.piece[x as usize][y as usize] = color_idx;
                }
                
                y += 1;
            }
            x += 1;
        }
    }

    // Rotate the piece 90 degrees.
    pub fn rotate(&mut self) {
        let temp: [[i32; 4]; 4] = self.piece.clone();

        let mut x: i32 = 0;
        while x < temp.len() as i32 {
            let mut y: i32 = 0;
            while y < temp[x as usize].len() as i32 {
                self.piece[x as usize][y as usize] = temp[(3 - y) as usize][x as usize];
                y += 1;
            }
            x += 1;
        }
    }
}
