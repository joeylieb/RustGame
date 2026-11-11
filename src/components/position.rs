pub struct Position {
    x: f32,
    y: f32
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            x,
            y
        }
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y
    }
}