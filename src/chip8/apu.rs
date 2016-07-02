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

    pub fn startsound(&mut self) {
        if self.beeping {return}
        println!("Now beeping");
        self.beeping = true;
    }

    pub fn stopsound(&mut self) {
        if !self.beeping {return}
        println!("No longer beeping");
        self.beeping = false;
    }
}