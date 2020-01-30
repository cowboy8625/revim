pub struct Dimensions {
    pub w: u16,
    pub h: u16,
}

impl Dimensions {
    pub fn new(w: u16, h: u16) -> Self {
        Self { w, h }
    }
}
