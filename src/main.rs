use std::fs::File;
use std::io::prelude::*;
use crate::chip8::*;

pub mod ram;
pub mod chip8;
pub mod cpu;

fn main() {
    let mut f = File::open("INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    f.read_to_end(&mut data).unwrap();
    // println!("SPCINV: {:?}", data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    loop {
        chip8.run_op();
    }
}