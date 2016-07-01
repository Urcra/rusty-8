extern crate rand;
extern crate sdl2;

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

    // Get these as input, or from ini file
    let cpu_hz = 600;
    let fps = 60;

    let width = 800;
    let height = 600;

    let mut chip_8 = Chip8::new(rom_buf, cpu_hz, fps, width, height);
    chip_8.run();
}
