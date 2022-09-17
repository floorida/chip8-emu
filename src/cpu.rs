use crate::{ram::Ram, chip8::Chip8};

#[derive(Debug)]
pub struct Cpu {
    vx: [u8; 16],
    counter: u16,
    i: u16

}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {  
            vx: [0; 16],
            counter: 0x200,
            i: 0
        }
    }

    fn set_vx_reg(&mut self, x: u8, byte: u8) {
        self.vx[x as usize] = byte;
    }

    fn set_mem_reg(&mut self, addr: u16) {
        self.i = addr;
    }
    
    fn debug_sprite(&self, h: u8, x: u8, y: u8, ram: &mut Ram) {
        println!("sprite at ({}, {}) of height {}", x, y, h);
    }

    pub fn run_ops(&mut self, ram: &mut Ram) {
        let high = ram.read_byte(self.counter) as u16; //removed shifting to the left here because futher checks come later
        let low = ram.read_byte(self.counter+1) as u16;
        let instruction: u16 = (high << 8) | low; // shift to combine into one 2 byte instruction
        let addr = instruction & 0b0000111111111111;
        let nibble = instruction & 0b0000000000001111;
        let x = (instruction & 0b0000111100000000) >> 8;
        let y = (instruction & 0b0000000011110000) >> 4;
        let byte = (instruction & 0b0000000011111111) as u8;
        println!("hi={:#X} lo={:#X} opcode={:#X} | addr={:?} nibble={:?} x={} y={} byte={:#b}", high, low, instruction, addr, nibble, x, y, byte);
        match (instruction & 0xF000) >> 12 {
            0x1 => {
                //set counter to addr
                self.counter = addr
            }
            0x6 => {
                //set vx to x
                self.set_vx_reg(x as u8, byte);
                self.counter+=2;
            }
            0xA => {
                //set i to addr
                self.set_mem_reg(addr);
                self.counter+=2;
            }
            0xD => {
                self.debug_sprite(nibble as u8,x as u8, y as u8, Chip8.ram);
            }

            _ => panic!("Inexistent instruction: {:#X} Counter: {}", instruction, self.counter)
        }
    }
}
