mod cpu;
mod gpu;
mod apu;

use self::cpu::CPU;

#[derive(Debug)]
pub struct Chip8 {
    cpu: CPU,
}


impl Chip8 {
    pub fn new(rom: Vec<u8>) -> Chip8 {
        Chip8 {
            cpu: CPU::new(rom),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.tick();
        }
    }


}