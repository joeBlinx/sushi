extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::time::Duration;
mod collide;
mod draw;
mod entity;
mod player;
use player::Player;
mod transfo_truc;
use collide::collide;
use transfo_truc::TransfoTruc;
fn update(player: &mut Player, transfo_trucs: &mut Vec<TransfoTruc>) {
    transfo_trucs.retain(|x| {
        if collide(x, player) {
            player.add_transfo_truc(x.clone());
            return false;
        }
        return true;
    });
}
pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Sushi", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(130, 130, 130));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut player = Player::new(0, 550);
    let mut transfo_trucs = vec![
        TransfoTruc::new(150, 550),
        TransfoTruc::new(210, 550),
        TransfoTruc::new(280, 550),
    ];
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
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            player.move_xy(1, 0);
        } else if keyboard_state.is_scancode_pressed(Scancode::Left) {
            player.move_xy(-1, 0);
        }
        update(&mut player, &mut transfo_trucs);
        draw::clear_canvas(&mut canvas);
        for transfo_truc in transfo_trucs.iter() {
            draw::draw_rectangle(transfo_truc, &mut canvas).unwrap();
        }
        draw::draw_rectangle(&player, &mut canvas).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }

    Ok(())
}
