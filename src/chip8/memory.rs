pub struct Memory {
    ram: [u8; 4096],
    registers: [[u8; 8]; 16],
    stack: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; 4096],
            registers: [[0; 8];16],
            stack: Vec::new()
        }
    }

    pub fn write_ram_bytes(&mut self, address: usize, value: u8) {
        self.ram[address] = value;
    }

    pub fn read_ram_bytes(&self, address: usize) -> u8 {
        self.ram[address]
    }
}