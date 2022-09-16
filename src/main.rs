extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;

pub fn main(){
    let sdl_context = sdl2::init().expect("Failed initializing SDL");
    let video_subsystem = sdl_context.video().expect("Failed loading SDL video");
    let window = video_subsystem.window("Tetrs 0.1.0", 300, 600)
        // Setting some flags
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window.");

    let mut canvas = window.into_canvas().build().expect("Failed to convert window into canvas");
    canvas.set_draw_color(Color::RGB(154,235,163));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    // We use a loop label to disambiguate between multiple loops. 
    // https://doc.rust-lang.org/book/ch03-05-control-flow.html?highlight=named%20loop%20arms#loop-labels-to-disambiguate-between-multiple-loops
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { ..} | 
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }
        // To avoid harassing the CPU, we only render 60 times per second.
        sleep(Duration::new(0,1_000_000_000u32 / 60));
    }
}