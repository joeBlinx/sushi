extern crate sdl2;

use draw::CanvasFont;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
mod canvas;
mod collide;
mod draw;
mod entity;
mod event;
mod player;
mod power;
mod sdl2_bridge;
mod shapes;
mod transfo_truc;
mod types;
mod world;
use crate::world::World;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = sdl2::ttf::init().unwrap();
    let arial_font = ttf_context.load_font("arial.ttf", 12).unwrap();
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Sushi", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut world = World::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let keyboard_state = event_pump.keyboard_state();
        world.event(&keyboard_state);
        world.update();
        world.draw(&mut CanvasFont {
            canvas: &mut canvas,
            font: &arial_font,
        });

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }

    Ok(())
}
