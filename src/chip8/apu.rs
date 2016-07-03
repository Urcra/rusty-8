extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};

pub struct Audio {
    device: sdl2::audio::AudioDevice<SquareWave>,
    beeping: bool,
}


impl Audio {
    pub fn new(output: sdl2::AudioSubsystem) -> Audio {


        // This beeping noise seems to work fine.

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = output.open_playback(None, &desired_spec, |spec| {

            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();


        Audio {
            device: device,
            beeping: false,
        }
    }

    pub fn startsound(&mut self) {
        if self.beeping {return}
        self.device.resume();
        self.beeping = true;
    }

    pub fn stopsound(&mut self) {
        if !self.beeping {return}
        self.device.pause();
        self.beeping = false;
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = match self.phase {
                0.0...0.5 => self.volume,
                _ => -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}