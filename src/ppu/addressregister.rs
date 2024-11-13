#[derive(Debug)]
pub struct AddressingRegister {
    value: (u8, u8),
    high_ptr: bool   
}

impl AddressingRegister {
    pub fn new() -> Self {
        AddressingRegister {
            value: (0, 0),
            high_ptr: true,
        }
    }

    fn set(&mut self, value: u16) {
        self.value.0 = (value >> 8) as u8;
        self.value.1 = (value & 0xff) as u8;
    }

    pub fn update(&mut self, data: u8) {
        if self.high_ptr {
            self.value.1 = data;
        } else {
            self.value.0 = data;
        }

        self.set(self.get() & 0x3fff);
    }

    pub fn increment(&mut self, inc: u8) {
        self.set(self.get().wrapping_add(inc as u16) & 0x3fff);
    }

    pub fn reset_latch(&mut self) {
        self.high_ptr = true;
    }

    pub fn get(&self) -> u16 {
        (self.value.0 as u16) << 8 | (self.value.1 as u16)
    }
}