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
use self::input::Keypad;

const DELAYTIMER: u32 = 60;

pub struct Chip8<'a> {
    cpu: CPU,
    display: Display<'a>,
    audio: Audio,
    context: sdl2::Sdl,

    cpu_hz: u32,
    fps: u32,
}


impl<'a> Chip8<'a> {
    pub fn new(rom: Vec<u8>, cpu_hz: u32, fps: u32, width: u32, height: u32) -> Chip8<'a> {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        Chip8 {
            cpu: CPU::new(rom),
            display: Display::new(video_subsystem, "test".into(), width, height),
            audio: Audio::new("test".into()),
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


                let test_mem = [[true, true, true, true, false, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false]; 32];

                self.display.draw_screen(&self.cpu.g_mem);
                //self.display.draw_screen(&test_mem);
            }

            if current.duration_since(last_cycle) > cycle_delay {
                last_cycle = Instant::now();

                self.cpu.tick();
            }

            // Should probably sleep for some amount of time here?
        }
    }


}