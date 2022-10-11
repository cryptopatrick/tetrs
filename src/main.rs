// Import the sdl2 and get access to everything it offers.
extern crate sdl2;

// Used to color window and textures.
use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator};
use sdl2::image::{LoadTexture, INIT_JPG};
// We'll use an even loop to keep the window, we intend to create, open.
use sdl2::event::Event;
// Remember that in sdl2, (0,0) is the top-left corner.
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::thread::sleep;

pub fn main(){
    // WINDOW //////////////////////////////////////////////////////////////////
    // We initialize an sdl2 context in order to get access to its library.
    let sdl_context = sdl2::init().expect("Failed initializing SDL");
    // Using the sdl2 context we fetch the video subsystem, and create a window
    // with the parameters: title, width, height.
    let video_subsystem = sdl_context.video().expect("Failed loading SDL video");
    
    let window = video_subsystem.window("Tetrs 0.1.0", 300, 600)
        // Setting some flags
        // Place window in the middle of the screen.
        .position_centered()
        // Tell sdl2 to use OpenGL to render content.
        .opengl()
        // Go ahead and build the window using the previous given parameters and flags.
        .build()
        .expect("Failed to create window.");
        
    // CANVAS //////////////////////////////////////////////////////////////////
    // In order to draw into the window we create a canvas, using the sdl2::video::window.
    // Constant to represent the size of a texture.
    let mut canvas = window
        // We transform the window into a canvas so that we can draw into it.
        .into_canvas()
        // We add support to render texture.
        .target_texture()
        // Enabling vertical synchronization limit.
        // https://docs.rs/sdl2/0.35.2/sdl2/render/struct.CanvasBuilder.html#method.present_vsync
        .present_vsync()
        .build()
        .expect("Failed to convert window into canvas");
    
        
        // TEXTURE /////////////////////////////////////////////////////////////////
        // We will create a texture baground that we can paste into the canvas of the window.
        // The texture will have a size of 32x32 and a color.
    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    const TEXTURE_SIZE: u32 = 32;
    
    //-> IMAGE 
    // InitFlags are passed to init() to control which subsystem functionality to load.
    sdl2::image::init(INIT_JPG).expect("Failed to load image context.");
    let image_texture = texture_creator.load_texture("assets/cat.jpg").expect("Failed to load picture.");
    
    let mut square_texture: Texture = 
        texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .expect("Failed to create texture");
        
    // Fill the canvas (window) with a greenish color.            
    canvas.set_draw_color(Color::RGB(154,235,163));
    canvas.clear();

    // Create a rect to hold the cat (position and size)
    let cat_rect: Rect = Rect::new(50,50,150,150);

    // Copy our texture into the window in the top left corner.
    canvas.copy(&image_texture, None, cat_rect).expect("Failed to load image into canvas.");
    canvas.copy(&square_texture, 
        None, 
        Rect::new(0,0,TEXTURE_SIZE, TEXTURE_SIZE))
        .expect("Failed to copy texture into canvas.");
        // We update the window display.
    canvas.present();

    // We'll use the Canvas struct to draw our square texture.
    let res = canvas.with_texture_canvas(&mut square_texture, |texture| {
        // .set_draw_color specifies which color to use when drawing.
        texture.set_draw_color(Color::RGB(0, 255, 0));
        texture.clear(); // Clear the texture so that it can be filled again.
    });


    // GAME LOOP ///////////////////////////////////////////////////////////////
    // We create an event manager by first getting a handler.
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
    // Next we create an infinite loop over events.
    // Note: we use a 'loop label' to disambiguate between multiple loops
    // and to make it easier to break an outside loop in a nested loop structure. 
    // https://doc.rust-lang.org/book/ch03-05-control-flow.html?highlight=named%20loop%20arms#loop-labels-to-disambiguate-between-multiple-loops
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                // Quit the program ife the user either presses the x of the window
                // or presses escape on the keyboard.
                Event::Quit {..} | 
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    println!("Escape key was pressed and application closed!");
                    break 'running
                },
                _ => {}
            }
        }
        // To avoid harassing the CPU, we only render 60 times per second.
        sleep(Duration::new(0,1_000_000_000u32 / 60));
    }
}