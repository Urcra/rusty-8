extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

pub struct Display<'a> {
    name: String,
    height: u32,
    width: u32,
    renderer: sdl2::render::Renderer<'a>,
}


impl<'a> Display<'a> {
    pub fn new(video: sdl2::VideoSubsystem, name: String, width: u32, height: u32) -> Display<'a> {
        let window = video.window(&name, width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();

        let mut disp = Display {
            name: name,
            height: height,
            width: width,
            renderer: renderer,
        };

        disp.adjust_scale();
        disp
    }

    pub fn test(&mut self) {
        self.renderer.set_draw_color(Color::RGB(255, 0, 0));
        self.renderer.clear();
        self.renderer.present();
    }

    pub fn adjust_scale(&mut self) {
        let w_scale = self.width / 64; 
        let h_scale = self.height / 32; 

        self.renderer.set_scale(w_scale as f32, h_scale as f32);
    }

    pub fn draw_screen(&mut self, g_mem: &[[bool; 64]; 32]) {
        for y in 0..32 {
            for x in 0..64 {
                if g_mem[y][x] {
                    self.renderer.set_draw_color(Color::RGB(0, 0, 0));
                } else {
                    self.renderer.set_draw_color(Color::RGB(255, 255, 255));
                }
                self.renderer.draw_point( Point::new(x as i32,y as i32));
            }
        }

        self.renderer.present();
    }
}