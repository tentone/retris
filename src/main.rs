use crate::tetris::Tetris;

mod vector2;
mod pieces;
mod piece;
mod tetris;
mod color;
mod board;

#[macroquad::main("Retris")]
async fn main() {
    let mut tetris: Tetris = Tetris::new();
    tetris.run().await;
}
