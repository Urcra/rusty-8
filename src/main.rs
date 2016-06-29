use std::io::prelude::*;
use std::env;
use std::fs::File;

#[derive(Debug)]
struct CPU {
    pc: u16,
    i_reg: u16,
    v_regs: [u8; 16],
    stack: Vec<u16>,

    memory: Vec<u8>,
}

impl CPU {
    fn new(rom: Vec<u8>) -> CPU {
        CPU {
            pc: 0x200,
            i_reg: 0,
            v_regs: [0; 16],
            stack: Vec::new(),
            memory: rom,
        }
    }

    fn tick(&mut self) {
        //fetch opcode
        let opcode = self.get_opcode();
        self.pc += 2;

        //match opcode
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                        0x0000 => self.op_00e0(opcode),
                        0x000E => self.op_00ee(opcode),
                        _      => println!("Unkown opcode: {:x}", opcode),
                    },
            0x1000 => self.op_1nnn(opcode),
            0x2000 => self.op_2nnn(opcode),
            0x3000 => self.op_3xnn(opcode),
            0x4000 => self.op_4xnn(opcode),
            0x5000 => self.op_5xy0(opcode),
            0x6000 => self.op_6xnn(opcode),
            0x7000 => self.op_7xnn(opcode),
            0x8000 => match opcode & 0x000F {
                        0x0000 => self.op_8xy0(opcode),
                        0x0001 => self.op_8xy1(opcode),
                        0x0002 => self.op_8xy2(opcode),
                        0x0003 => self.op_8xy3(opcode),
                        0x0004 => self.op_8xy4(opcode),
                        0x0005 => self.op_8xy5(opcode),
                        0x0006 => self.op_8xy6(opcode),
                        0x0007 => self.op_8xy7(opcode),
                        0x000E => self.op_8xye(opcode),
                        _      => println!("Unkown opcode: {:x}", opcode),
                    },
            0x9000 => self.op_9xy0(opcode),
            0xA000 => self.op_annn(opcode),
            0xB000 => self.op_bnnn(opcode),
            0xC000 => self.op_cxnn(opcode),
            0xD000 => self.op_dxyn(opcode),
            0xE000 => match opcode & 0x00FF {
                        0x009E => self.op_ex9e(opcode),
                        0x00A1 => self.op_exa1(opcode),
                        _      => println!("Unkown opcode: {:x}", opcode),
                    },
            0xF000 => match opcode & 0x00FF {
                        0x0007 => self.op_fx07(opcode),
                        0x000A => self.op_fx0a(opcode),
                        0x0015 => self.op_fx15(opcode),
                        0x0018 => self.op_fx18(opcode),
                        0x001E => self.op_fx1e(opcode),
                        0x0029 => self.op_fx29(opcode),
                        0x0033 => self.op_fx33(opcode),
                        0x0055 => self.op_fx55(opcode),
                        0x0065 => self.op_fx65(opcode),
                        _      => println!("Unkown opcode: {:x}", opcode),
                    },
            _      => println!("Unkown opcode: {:x}", opcode),
        };

        //Increment PC

        self.pc += 2;
    }

    fn get_opcode(&mut self) -> u16 {
        let mut opcode = self.memory[self.pc as usize] as u16;
        opcode <<= 8;
        opcode | self.memory[(self.pc as usize) + 1] as u16
    }

    fn op_00e0(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_00ee(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_1nnn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_2nnn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_3xnn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_4xnn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_5xy0(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_6xnn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_7xnn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_8xy0(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_8xy1(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_8xy2(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_8xy3(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_8xy4(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_8xy5(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_8xy6(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_8xy7(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_8xye(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_9xy0(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_annn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_bnnn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_cxnn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_dxyn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_ex9e(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_exa1(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_fx07(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_fx0a(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_fx15(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_fx18(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_fx1e(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_fx29(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_fx33(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_fx55(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }

    fn op_fx65(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:x}", opcode);
    }
}



fn main() {

    let rom_name = env::args().nth(1).unwrap();
    let mut rom_fd = File::open(rom_name).unwrap();

    let mut rom_buf = vec![0; 0x200];
    rom_fd.read_to_end(&mut rom_buf);

    let mut cpu = CPU::new(rom_buf);

    cpu.tick();


    println!("{:x}", &cpu.get_opcode());
}
