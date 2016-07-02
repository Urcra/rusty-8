extern crate sdl2;

use sdl2::keyboard::Keycode;

const KEY_0: u8 = 0x0;
const KEY_1: u8 = 0x1;
const KEY_2: u8 = 0x2;
const KEY_3: u8 = 0x3;
const KEY_4: u8 = 0x4;
const KEY_5: u8 = 0x5;
const KEY_6: u8 = 0x6;
const KEY_7: u8 = 0x7;
const KEY_8: u8 = 0x8;
const KEY_9: u8 = 0x9;

const KEY_A: u8 = 0xA;
const KEY_B: u8 = 0xB;
const KEY_C: u8 = 0xC;
const KEY_D: u8 = 0xD;
const KEY_E: u8 = 0xE;
const KEY_F: u8 = 0xF;

pub struct Keypad {
    pub last_pressed: Option<u8>,
    pub key_state: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad{
        Keypad {
            last_pressed: None,
            key_state: [false; 16],
        }
    }

    pub fn key_pressed(&mut self, key: Keycode) {
        match key {
            Keycode::Num0 => self.set_key(KEY_0),
            Keycode::Num1 => self.set_key(KEY_1),
            Keycode::Num2 => self.set_key(KEY_2),
            Keycode::Num3 => self.set_key(KEY_3),
            Keycode::Num4 => self.set_key(KEY_4),
            Keycode::Num5 => self.set_key(KEY_5),
            Keycode::Num6 => self.set_key(KEY_6),
            Keycode::Num7 => self.set_key(KEY_7),
            Keycode::Num8 => self.set_key(KEY_8),
            Keycode::Num9 => self.set_key(KEY_9),
            Keycode::A => self.set_key(KEY_A),
            Keycode::B => self.set_key(KEY_B),
            Keycode::C => self.set_key(KEY_C),
            Keycode::D => self.set_key(KEY_D),
            Keycode::E => self.set_key(KEY_E),
            Keycode::F => self.set_key(KEY_F),
            _ => {},
        }
    }

    pub fn key_released(&mut self, key: Keycode) {
        match key {
            Keycode::Num0 => self.unset_key(KEY_0),
            Keycode::Num1 => self.unset_key(KEY_1),
            Keycode::Num2 => self.unset_key(KEY_2),
            Keycode::Num3 => self.unset_key(KEY_3),
            Keycode::Num4 => self.unset_key(KEY_4),
            Keycode::Num5 => self.unset_key(KEY_5),
            Keycode::Num6 => self.unset_key(KEY_6),
            Keycode::Num7 => self.unset_key(KEY_8),
            Keycode::Num8 => self.unset_key(KEY_9),
            Keycode::Num9 => self.unset_key(KEY_9),
            Keycode::A => self.unset_key(KEY_A),
            Keycode::B => self.unset_key(KEY_B),
            Keycode::C => self.unset_key(KEY_C),
            Keycode::D => self.unset_key(KEY_D),
            Keycode::E => self.unset_key(KEY_E),
            Keycode::F => self.unset_key(KEY_F),
            _ => {},
        }
    }

    fn set_key(&mut self, key: u8) {
        self.key_state[key as usize] = true;
        self.last_pressed = Some(key);
    }

    fn unset_key(&mut self, key: u8) {
        self.key_state[key as usize] = false;

        if self.last_pressed == Some(key) {
            self.last_pressed = None;
        }
    }
}