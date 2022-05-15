use crate::collide::collide;
use crate::draw;
use crate::entity::Movable;
use crate::player::Player;
use crate::player::Power;
use crate::transfo_truc::TransfoTruc;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
pub struct World {
    player: Player,
    transfo_trucs: Vec<TransfoTruc>,
}
impl World {
    pub fn new() -> World {
        World {
            player: Player::new(0, 500),
            transfo_trucs: vec![
                TransfoTruc::new(150, 550),
                TransfoTruc::new(210, 550),
                TransfoTruc::new(280, 550),
            ],
        }
    }
    pub fn update(&mut self) {
        self.transfo_trucs.retain(|x| {
            if collide(x, &mut self.player) {
                self.player.add_transfo_truc(x.clone());
                return false;
            }
            return true;
        });
    }
    pub fn event(&mut self, event_pump: &sdl2::EventPump) {
        let keyboard_state = event_pump.keyboard_state();
        if keyboard_state.is_scancode_pressed(Scancode::Right) {
            self.player.move_xy(1, 0);
        } else if keyboard_state.is_scancode_pressed(Scancode::Left) {
            self.player.move_xy(-1, 0);
        } else if keyboard_state.is_scancode_pressed(Scancode::E) {
            self.player.trigger_power(Power::SWORD);
        } else if keyboard_state.is_scancode_pressed(Scancode::Q) {
            self.player.use_power();
        }
    }
    pub fn draw(&mut self, canvas: &mut Canvas<Window>, font: &Font) {
        canvas.set_draw_color(Color::RGB(130, 130, 130));
        canvas.clear();
        for transfo_truc in self.transfo_trucs.iter() {
            draw::draw_rectangle(transfo_truc, canvas).unwrap();
        }
        draw::draw_rectangle(&self.player, canvas).unwrap();
        match self.player.get_power() {
            None => {}
            Some(power) => draw::draw_rectangle_dyn(power.as_ref(), canvas).unwrap(),
        }
        draw::display_text(
            canvas,
            &font,
            &self.player.get_transfo_trucs_count().to_string(),
        );
        canvas.present();
    }
}
