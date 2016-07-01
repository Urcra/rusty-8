extern crate sdl2;

use sdl2::pixels::Color;

pub struct Audio {
    output: String,
    beeping: bool,
}


impl Audio {
    pub fn new(output: String) -> Audio {
        Audio {
            output: output,
            beeping: false,
        }
    }

    pub fn startsound(&self) {
        if self.beeping {return}
        println!("Now beeping");
    }

    pub fn stopsound(&self) {
        if !self.beeping {return}
        println!("No longer beeping");
    }
}