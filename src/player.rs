extern crate sdl2;
use crate::draw::Draw;
use crate::entity::EntityMovable;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Player {
    entity: EntityMovable,
}
impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            entity: EntityMovable::new(x, y, Color::BLUE),
        }
    }
    pub fn move_xy(&mut self, x: i32, y: i32) {
        self.entity.move_xy(x, y)
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
