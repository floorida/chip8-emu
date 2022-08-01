use crate::ram::Ram;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16

}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {  
            vx: [0; 16],
            pc: 0x200,
            i: 0
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {

    }
}