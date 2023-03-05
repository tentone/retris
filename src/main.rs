use macroquad::prelude::*;

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
    board: TetrisBoard,
    
    // Status
    running: bool,

    // Store the current game score
    score: i32,
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




#[macroquad::main("Retris")]
async fn main() {
    // Flag to indicate if the game is running
    let mut running = true;

    // Pause control flag
    let mut paused = false;

    // Store the current game score
    let mut score = 0;

    // Count how many frames ellapsed since the beginning of the game
    let mut frame = 0;

    // Board size
    let size: Vector2i = Vector2i::new(10, 20);

    let pieces: [[[i32; 4]; 4]; 7] = [
        [
            [0, 0, 0, 0],
            [1, 1, 1, 1],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ],
        [
            [1, 0, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ],
        [
            [0, 0, 0, 0],
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0]
        ],
        [
            [0, 0, 0, 1],
            [0, 1, 1, 1],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ],
        [
            [0, 1, 0, 0],
            [1, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ],
        [
            [0, 0, 1, 1],
            [0, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ],
        [
            [1, 1, 0, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ]
    ];

    // Current board status
    let mut board: [[i32; 10]; 20] = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 2, 3, 3, 4, 4, 5, 6, 7],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 0, 0, 0],
        [1, 1, 2, 3, 3, 4, 4, 5, 6, 7],
        [0, 0, 0, 0, 4, 2, 0, 0, 0, 0],
        [0, 0, 0, 2, 3, 0, 0, 0, 0, 0],
        [1, 1, 2, 3, 3, 4, 4, 5, 6, 7],
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


    // Piece position and type
    let mut piece_pos: Vector2i = Vector2i::new(4, 0);
    let mut piece: [[i32; 4]; 4] = pieces[0].clone();

    
    while running {
        // ------------------- Update Logic -------------------
        // let ms = SystemTime::now().duration_since(UNIX_EPOCH).as_millis();
        // println!("{0}", ms);

        // let ms: time::Duration = time::Duration::from_millis(1000);
        // thread::sleep(ms);

        // Pause
        if is_key_pressed(KeyCode::P) {
            paused = !paused;
        }


        // Rotate Piece
        if is_key_pressed(KeyCode::Up) {
            println!("Up is down");

            let temp: [[i32; 4]; 4] = piece.clone();
            
            piece[0][0] = temp[3][0];
            piece[0][1] = temp[2][0];
            piece[0][2] = temp[1][0];
            piece[0][3] = temp[0][0];

            piece[1][0] = temp[3][1];
            piece[1][1] = temp[2][1];
            piece[1][2] = temp[1][1];
            piece[1][3] = temp[0][1];
            
            piece[2][0] = temp[3][2];
            piece[2][1] = temp[2][2];
            piece[2][2] = temp[1][2];
            piece[2][3] = temp[0][2];

            piece[3][0] = temp[3][3];
            piece[3][1] = temp[2][3];
            piece[3][2] = temp[1][3];
            piece[3][3] = temp[0][3];
            


            // TODO <ROTATE PROPERLY>
            // let mut x: i32 = 0;
            // while x < temp.len() as i32 {
            //     let mut y: i32 = 0;
            //     while y < temp[x as usize].len() as i32 {
                    
            //         piece[x as usize][y as usize] = temp[y as usize][x as usize];
            //         y += 1;
            //     }
            //     x += 1;
            // }

        }
        // Move Piece left
        if is_key_pressed(KeyCode::Left) {
            println!("left is down");
            piece_pos.x -= 1;
        }
        if is_key_pressed(KeyCode::Right) {
            println!("right is down");
            piece_pos.x += 1;
        }

        // Place the piece where it is (just for testing)
        if is_key_pressed(KeyCode::F) {
            let mut piece_idx: i32 = macroquad::rand::gen_range::<i32>(0, pieces.len() as i32);
            let mut color_idx: i32 = macroquad::rand::gen_range::<i32>(1, colors.len() as i32);
            piece = pieces[piece_idx as usize].clone();

            // Set color
            let mut x: i32 = 0;
            while x < piece.len() as i32 {
                let mut y: i32 = 0;
                while y < piece[x as usize].len() as i32 {
                    if piece[x as usize][y as usize] != 0 {
                        piece[x as usize][y as usize] = color_idx;
                    }
                    
                    y += 1;
                }
                x += 1;
            }

            // Force place piece in board
            // TODO <FORCE PLACE PIECE>
        }

        // Move Piece faster
        let mut speedup: bool = false;
        if is_key_down(KeyCode::Down) {
            println!("down is down");
            speedup = true;
        }
        

        // Move piece down
        if frame % 40 == 0 || speedup && frame % 5 == 0 {
            // Check if the piece can move down further
            let mut piece_can_move: bool = true;
            // TODO

            // Move the piece down
            if piece_can_move {
                piece_pos.y += 1;
            }

            // Check the status of the board
            let mut y: i32 = 0;
            while y < size.y {

                // Check if any row in the board is full
                let mut row_full: bool = true;
                let mut x: i32 = 0;
                while x < size.x {
                    let value: i32 = board[y as usize][x as usize];
                    if value == 0 {
                        row_full = false;
                        break;
                    }
                    x += 1;
                }

                // If row is full remove from board and increase score
                if row_full {
                    println!("Row is full");

                    // Move rows above this one bellow
                    let mut yy: i32 = y;
                    while yy > 1 {
                        let mut x: i32 = 0;
                        while x < size.x {
                            board[yy as usize][x as usize] = board[(yy - 1) as usize][x as usize];
                            x += 1;
                        }

                        yy -= 1;
                    }

                    // Clear first row
                    let mut x: i32 = 0;
                    while x < size.x {
                        board[0][x as usize] = 0;
                        x += 1;
                    }

                    score += 1;
                }
                y += 1;
            }
        }

        // ------------------- Render code -------------------
        clear_background(BLACK);

        let width: f32 = screen_width();
        let height: f32 = screen_height();
        
        let block: f32 = height / size.y as f32;
        let origin: f32 = width as f32 / 2.0 - (size.x as f32 * block) / 2.0;

        // Base board
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


        // Moving piece
        let mut x: i32 = 0;
        while x < 4 {
            let mut y: i32 = 0;
            while y < 4 {
                draw_rectangle(origin + block * (piece_pos.x + x) as f32, block * (piece_pos.y + y) as f32, block as f32, block, colors[piece[y as usize][x as usize] as usize]);
                y += 1;
            }
            x += 1;
        }

        // Draw text
        draw_text(&(String::from("Frame: ") + &frame.to_string()), 10.0, 25.0, 30.0, macroquad::color::WHITE);
        draw_text(&(String::from("Score: ") + &score.to_string()), 10.0, 50.0, 30.0, macroquad::color::WHITE);

        // Pause message
        if paused {
            draw_rectangle(0.0, 0.0, width, height, macroquad::color::Color{r: 0.0, g: 0.0, b: 0.0, a: 0.4});
            draw_text("Paused", width / 2.0, height / 2.0, 40.0, macroquad::color::WHITE);
        }
        

        frame += 1;

        // Close application
        if is_key_down(KeyCode::Escape) {
            return;
        }

        next_frame().await;
    }
}
