pub mod vector2;

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

    /**
     * Set the value of the vector.
     */
    pub fn set(&mut self, x: i32, y: i32) -> () {
        self.x = x;
        self.y = y;
    }
}