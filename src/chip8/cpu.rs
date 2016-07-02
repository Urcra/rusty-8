extern crate rand;

use rand::random;

use super::input::Keypad;

pub struct CPU {
    pc: u16,
    i_reg: u16,
    v_regs: [u8; 16],
    stack: Vec<u16>,

    memory: Vec<u8>,

    pub delay_timer: u8,
    pub sound_timer: u8,

    pub g_mem: [[bool; 64]; 32],

    pub key_state: Keypad,
}

// Remember overflowing ops for u8's

impl CPU {
    pub fn new(rom: Vec<u8>) -> CPU {
        CPU {
            pc: 0x200,
            i_reg: 0,
            v_regs: [0; 16],
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            memory: rom,
            g_mem: [[false; 64]; 32],

            key_state: Keypad::new(),
        }
    }

    pub fn tick(&mut self) {
        //fetch opcode
        self.print_state();
        let opcode = self.get_opcode();
        self.pc += 2;
        //match opcode
        match opcode & 0xF000 {
            0x0000 => match opcode & 0x000F {
                        0x0000 => self.op_00e0(opcode),
                        0x000E => self.op_00ee(opcode),
                        _      => println!("Unkown opcode: {:#X}", opcode),
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
                        _      => println!("Unkown opcode: {:#X}", opcode),
                    },
            0x9000 => self.op_9xy0(opcode),
            0xA000 => self.op_annn(opcode),
            0xB000 => self.op_bnnn(opcode),
            0xC000 => self.op_cxnn(opcode),
            0xD000 => self.op_dxyn(opcode),
            0xE000 => match opcode & 0x00FF {
                        0x009E => self.op_ex9e(opcode),
                        0x00A1 => self.op_exa1(opcode),
                        _      => println!("Unkown opcode: {:#X}", opcode),
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
                        _      => println!("Unkown opcode: {:#X}", opcode),
                    },
            _      => println!("Unkown opcode: {:#X}", opcode),
        };
    }

    fn get_opcode(&self) -> u16 {
        let mut opcode = self.memory[self.pc as usize] as u16;
        opcode <<= 8;
        opcode | self.memory[(self.pc as usize) + 1] as u16
    }

    fn print_state(&self) {
        println!("PC: {:?}", self.pc);
        println!("IP: {:?}", self.i_reg);
        let opcode = self.get_opcode();
        println!("Decoding op: {:#X}", opcode);
    }



    fn op_00e0(&mut self, opcode: u16) {

        for y in 0..32 {
            for x in 0..64 {
                self.g_mem[y][x] = false;
            }
        }
        //panic!("UNIMPLEMENTED OPCODE: {:#X}", opcode);
    }

    fn op_00ee(&mut self, opcode: u16) {
        self.pc = self.stack.pop().unwrap();
    }

    fn op_1nnn(&mut self, opcode: u16) {
        self.pc = opcode & 0x0FFF;
    }

    fn op_2nnn(&mut self, opcode: u16) {
        self.stack.push(self.pc);
        self.pc = opcode & 0x0FFF;
    }

    fn op_3xnn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;

        if self.v_regs[x] == nn {
            self.pc += 2;
        }
    }

    fn op_4xnn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;

        if self.v_regs[x] != nn {
            self.pc += 2;
        }
    }

    fn op_5xy0(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        if self.v_regs[x] == self.v_regs[y] {
            self.pc += 2;
        }
    }

    fn op_6xnn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        self.v_regs[x] = nn;
    }

    fn op_7xnn(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        self.v_regs[x] = self.v_regs[x].wrapping_add(nn);
    }

    fn op_8xy0(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.v_regs[x] = self.v_regs[y];
    }

    fn op_8xy1(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.v_regs[x] = self.v_regs[x] | self.v_regs[y];
    }

    fn op_8xy2(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.v_regs[x] = self.v_regs[x] & self.v_regs[y];
    }

    fn op_8xy3(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.v_regs[x] = self.v_regs[x] ^ self.v_regs[y];
    }

    fn op_8xy4(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let (res, overflow) = self.v_regs[x].overflowing_add(self.v_regs[y]);

        self.v_regs[x] = res;
        self.v_regs[0xF] = overflow as u8;
    }

    fn op_8xy5(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let (res, overflow) = self.v_regs[x].overflowing_sub(self.v_regs[y]);

        self.v_regs[x] = res;
        self.v_regs[0xF] = overflow as u8;
    }

    fn op_8xy6(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:#X}", opcode);
    }

    fn op_8xy7(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let (res, overflow) = self.v_regs[y].overflowing_sub(self.v_regs[x]);

        self.v_regs[x] = res;
        self.v_regs[0xF] = overflow as u8;
    }

    fn op_8xye(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:#X}", opcode);
    }

    fn op_9xy0(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        if self.v_regs[x] != self.v_regs[y] {
            self.pc += 2;
        }

    }

    fn op_annn(&mut self, opcode: u16) {
        self.i_reg = opcode & 0x0FFF;
    }

    fn op_bnnn(&mut self, opcode: u16) {
        panic!("UNIMPLEMENTED OPCODE: {:#X}", opcode);
    }

    fn op_cxnn(&mut self, opcode: u16) {
        let random_number = rand::random::<u8>();

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;

        self.v_regs[x] = random_number & nn;
    }

    fn op_dxyn(&mut self, opcode: u16) {


        
        let reg_x = ((opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((opcode & 0x00F0) >> 4) as usize;

        let height = (opcode & 0x000F) as usize;
        let coordx = self.v_regs[reg_x] as usize;
        let coordy = self.v_regs[reg_y] as usize;

        if coordx > 0x3F || coordy > 0x1F {
            panic!("");
        }

        

        let mut flipped = false;


        for y in 0..height {
            let mut byte = self.memory[self.i_reg as usize + y];
            let bits = bits_in_byte(byte);
            for x in 0..8 {
                let mask = 0b_1000_0000_u8 >> x; 
                let curr_bit = byte & mask != 0;
                //let curr_bit = (byte & 0x1) != 0;
                //let curr_bit = bits[x];

                let x_index = (x + coordx) % 64;
                let y_index = (y + coordy) % 32;


                if x_index > 63 || y_index > 31 {
                    println!("something went wrong");
                    println!("x: {:?} y: {:?}", x_index, y+coordy);
                    panic!("");
                }
                flipped = flipped || curr_bit && self.g_mem[y_index][x_index];
                self.g_mem[y_index][x_index] ^= curr_bit;
                
                //byte >>= 1;
            }
        }

        self.v_regs[0xF] = flipped as u8;
        if flipped {
            self.v_regs[0xF] = 0x01;
        } else {
            self.v_regs[0xF] = 0x00;
        }
        


        /*
        for yline in 0..height {
            let data = self.memory[self.i_reg as usize + yline];
            let xpixelinv = 7;
            for xpixel in 0..8 {
                let mask = 1 << xpixelinv;

                if (data & mask) != 0 {
                    let x = coordx + xpixel;
                    let y = coordy + yline;
                    println!("x,y{:?}", (x,y));
                    if self.g_mem[x][y] {
                        self.v_regs[0xF] = 1;
                    }
                    self.g_mem[x][y] ^= true;
                }
            }
        }
        */

        fn bits_in_byte(byte: u8) -> Vec<bool>{
            let mut bits = Vec::new();
            bits.push(((byte & 0b10000000) >> 7) != 0);
            bits.push(((byte & 0b01000000) >> 6) != 0);
            bits.push(((byte & 0b00100000) >> 5) != 0);
            bits.push(((byte & 0b00010000) >> 4) != 0);
            bits.push(((byte & 0b00001000) >> 3) != 0);
            bits.push(((byte & 0b00000100) >> 2) != 0);
            bits.push(((byte & 0b00000010) >> 1) != 0);
            bits.push(((byte & 0b00000001) >> 0) != 0);

            bits
        }
    }

    fn op_ex9e(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let num = self.v_regs[x] as usize;

        if self.key_state.key_state[num] {
            self.pc += 2;
        }
    }

    fn op_exa1(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let num = self.v_regs[x] as usize;

        if !self.key_state.key_state[num] {
            self.pc += 2;
        }
    }

    fn op_fx07(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        self.v_regs[x] = self.delay_timer;
    }

    fn op_fx0a(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        match self.key_state.last_pressed {
            Some(key) => self.v_regs[x] = key,
            None      => self.pc -= 2,
        };
    }

    fn op_fx15(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        self.delay_timer = self.v_regs[x];
    }

    fn op_fx18(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        self.sound_timer = self.v_regs[x];
    }

    fn op_fx1e(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        self.i_reg = self.i_reg.wrapping_add(self.v_regs[x] as u16);
    }

    fn op_fx29(&mut self, opcode: u16) {
        // may be wrong
        let x = ((opcode & 0x0F00) >> 8) as usize;

        self.i_reg = self.v_regs[x] as u16 * 5;
    }

    fn op_fx33(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        let hundreds = self.v_regs[x] / 100;
        let tens = (self.v_regs[x] / 10) % 10;
        let units = self.v_regs[x] % 10;

        self.memory[self.i_reg as usize] = hundreds;
        self.memory[(self.i_reg as usize) + 1] = tens;
        self.memory[(self.i_reg as usize) + 2] = units;
    }

    fn op_fx55(&mut self, opcode: u16) {

        let x = ((opcode & 0x0F00) >> 8) as usize;

        for i in 0..(x+1) {
            self.memory[(self.i_reg + i as u16) as usize] = self.v_regs[i as usize];
        }

        self.i_reg = self.i_reg.wrapping_add((x + 1) as u16);


    }

    fn op_fx65(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        for i in 0..(x+1) {
            self.v_regs[i as usize] = self.memory[(self.i_reg + i as u16) as usize];
        }

        self.i_reg = self.i_reg.wrapping_add((x + 1) as u16);
    }
}