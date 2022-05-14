extern crate sdl2;
use crate::collide::{Collider, Sphere};
use crate::draw::Draw;
use crate::entity::EntityMovable;
use crate::transfo_truc::TransfoTruc;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Player {
    entity: EntityMovable,
    transfo_trucs: Vec<TransfoTruc>,
}
impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            entity: EntityMovable::new(x, y, Color::BLUE),
            transfo_trucs: Vec::<TransfoTruc>::new(),
        }
    }
    pub fn move_xy(&mut self, x: i32, y: i32) {
        self.entity.move_xy(x, y)
    }
    pub fn add_transfo_truc(&mut self, transfo_truc: TransfoTruc) {
        self.transfo_trucs.push(transfo_truc);
    }
    pub fn get_transfo_trucs_count(&self) -> usize {
        self.transfo_trucs.len()
    }
}
impl Draw for Player {
    fn get_rect(&self) -> Rect {
        self.entity.get_rect()
    }
    fn get_color(&self) -> Color {
        self.entity.get_color()
    }
}
impl Collider for Player {
    fn get_collider(&self) -> Sphere {
        self.entity.get_collider()
    }
}
