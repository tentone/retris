pub struct Vector2i {
    pub x: i32,
    pub y: i32
}

impl Clone for Vector2i {
    fn clone(&self) -> Vector2i {
        return Vector2i {
            x: self.x,
            y: self.y,
        }
    }
}

impl Copy for Vector2i { }

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
