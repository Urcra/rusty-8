extern crate sdl2;

use sdl2::pixels::Color;

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

        Display {
            name: name,
            height: height,
            width: width,
            renderer: renderer,
        }
    }

    pub fn test(&mut self) {
        self.renderer.set_draw_color(Color::RGB(255, 0, 0));
        self.renderer.clear();
        self.renderer.present();
    }

    pub fn draw_screen(&mut self, g_mem: &[[u8; 64]; 32]) {

    }
}