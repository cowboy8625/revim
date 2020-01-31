pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn move_up(&mut self, jump: u16, clamp: u16) {
        if self.y > clamp {
            self.y -= jump;
        }
    }

    pub fn move_down(&mut self, jump: u16, clamp: u16) {
        if self.y < clamp {
            self.y += jump;
        }
    }

    pub fn move_left(&mut self, jump: u16, clamp: u16) {
        if self.x > clamp {
            self.x -= jump;
        }
    }

    pub fn move_right(&mut self, jump: u16, clamp: u16) {
        if self.x < clamp {
            self.x += jump;
        }
    }
}
