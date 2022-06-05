use crate::canvas::Canvas;
use crate::collide::collide;
use crate::entity::Movable;
use crate::event::{Event, Key};
use crate::player::Player;
use crate::power;
use crate::transfo_truc::TransfoTruc;
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
    pub fn event(&mut self, event: &impl Event) {
        if event.is_pressed(Key::Right) {
            self.player.move_xy(1, 0);
        } else if event.is_pressed(Key::Left) {
            self.player.move_xy(-1, 0);
        } else if event.is_pressed(Key::E) {
            self.player.trigger_power::<power::Sword>();
        } else if event.is_pressed(Key::Q) {
            self.player.use_power();
        }
    }
    pub fn draw(&mut self, canvas: &mut impl Canvas) {
        canvas.clear();
        for transfo_truc in self.transfo_trucs.iter() {
            canvas.draw_rectangle(transfo_truc);
        }
        canvas.draw_rectangle(&self.player);
        if let Some(power) = &self.player.get_power() {
            canvas.draw_power(power.as_ref());
        }
        canvas.draw_text(&self.player.get_transfo_trucs_count().to_string());
        canvas.present();
    }
}
