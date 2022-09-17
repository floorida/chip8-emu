use crate::ram::Ram;
use crate::cpu::Cpu;
pub struct Chip8 {
    ram: Ram,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new()
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        for i in 0..rom.len() {
            self.ram.write_byte(0x200+i as u16, rom[i])
        }
    } 

    pub fn run_op(&mut self) {
        self.cpu.run_ops(&mut self.ram);
        println!("{:?}", self.cpu);
    }
}