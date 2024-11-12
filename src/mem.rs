
pub trait Mem {
    fn mem_read_u8(&self, addr: u16) -> u8;
    fn mem_write_u8(&mut self, addr: u16, value: u8);
    
    fn mem_read_u16(&self, addr: u16) -> u16 {
        (self.mem_read_u8(addr.wrapping_add(1)) as u16) << 8 | self.mem_read_u8(addr) as u16
    }

    fn mem_write_u16(&mut self, addr: u16, value: u16) {
        self.mem_write_u8(addr, (value & 0xff) as u8);
        self.mem_write_u8(addr.wrapping_add(1), (value >> 8) as u8);
    }
} 