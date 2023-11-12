pub struct Bootrom {
    rom: Box<[u8]>,
}
imple Bootrom {
    pub fun new(rom: Box<[u8]>) -> Self {
        Self {
            rom
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}