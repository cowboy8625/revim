pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn to_index(&self, width: u16) -> u16 {
        self.y * width + self.x
    }
}
