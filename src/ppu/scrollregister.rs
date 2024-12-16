#[derive(Debug)]

pub struct ScrollRegister {
    pub scroll_x: u8,
    pub scroll_y: u8,
    changing_y: bool
}

impl ScrollRegister {
    pub fn new() -> Self {
        ScrollRegister {
            scroll_x: 0,
            scroll_y: 0,
            changing_y: false
        }
    }

    pub fn write(&mut self, value: u8) {
        if self.changing_y {
            self.scroll_y = value;
        } else {
            self.scroll_x = value;
        }
        self.changing_y = !self.changing_y;
    } 

    pub fn reset_latch(&mut self) {
        self.changing_y = false;
    }
}
