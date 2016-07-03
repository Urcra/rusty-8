extern crate sdl2;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod cpu;
mod gpu;
mod apu;
mod input;

use self::cpu::CPU;
use self::gpu::Display;
use self::apu::Audio;

const DELAYTIMER: u32 = 60;
const ROM_START: usize = 0x200;
const RAM_SIZE: usize = 4000;

pub struct Chip8<'a> {
    cpu: CPU,
    display: Display<'a>,
    audio: Audio,
    context: sdl2::Sdl,

    cpu_hz: u32,
    fps: u32,
}


impl<'a> Chip8<'a> {
    pub fn new(rom_name: &str, rom: Vec<u8>, cpu_hz: u32, fps: u32, width: u32, height: u32) -> Chip8<'a> {

        let mut memory = vec![0; RAM_SIZE];

        for i in 0..FONTSET.len() {
            memory[i] = FONTSET[i];
        }

        for i in 0..rom.len() {
            memory[i + ROM_START] = rom[i];
        }


        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let audio_subsystem = sdl_context.audio().unwrap();

        Chip8 {
            cpu: CPU::new(memory),
            display: Display::new(video_subsystem, format!("Rusty8 - {}", rom_name).into(), width, height),
            audio: Audio::new(audio_subsystem),
            context: sdl_context,
            cpu_hz: cpu_hz,
            fps: fps,
        }
    }

    pub fn run(&mut self) {

        let mut event_pump = self.context.event_pump().unwrap();


        let timer_delay = Duration::new(0, ((1_f64 / DELAYTIMER as f64) * 1_000_000_000_f64) as u32);
        let frame_delay = Duration::new(0, ((1_f64 / self.fps as f64) * 1_000_000_000_f64) as u32);
        let cycle_delay = Duration::new(0, ((1_f64 / self.cpu_hz as f64) * 1_000_000_000_f64) as u32);

        let mut last_timer = Instant::now();
        let mut last_frame = Instant::now();
        let mut last_cycle = Instant::now();


        'running: loop {
            let current = Instant::now();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    Event::KeyDown { keycode: Some(x), .. } => self.cpu.key_state.key_pressed(x),
                    Event::KeyUp { keycode: Some(x), .. } => self.cpu.key_state.key_released(x),
                    _ => {}
                }
            }

            if current.duration_since(last_timer) > timer_delay {
                last_timer = Instant::now();

                if self.cpu.delay_timer > 0 {
                    self.cpu.delay_timer -= 1;
                }

                if self.cpu.sound_timer > 0 {
                    self.cpu.sound_timer -= 1;
                    self.audio.startsound();
                } else {
                    self.audio.stopsound();
                }

            }

            if current.duration_since(last_frame) > frame_delay {
                last_frame = Instant::now();
                self.display.draw_screen(&self.cpu.g_mem);
            }

            if current.duration_since(last_cycle) > cycle_delay {
                last_cycle = Instant::now();
                self.cpu.tick();
            }

            // Should probably sleep for some amount of time here?
        }
    }
}

static FONTSET: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70,
                            0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0,
                            0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
                            0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40,
                            0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0,
                            0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
                            0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
                            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80];