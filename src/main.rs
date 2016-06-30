extern crate rand;

mod chip8;
use chip8::Chip8;

use std::io::prelude::*;
use std::env;
use std::fs::File;

fn main() {

    let rom_name = env::args().nth(1).unwrap();
    let mut rom_fd = File::open(rom_name).unwrap();

    let mut rom_buf = vec![0; 0x200];
    rom_fd.read_to_end(&mut rom_buf);

    let mut chip_8 = Chip8::new(rom_buf);
    chip_8.run();
}
