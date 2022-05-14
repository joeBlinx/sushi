use crate::collide::{Collider, Sphere};
use crate::draw::Draw;
use crate::entity::{EntityMovable, Movable};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub trait PowerTrait: Collider + Draw + Movable {}
pub struct Sword {
    entity: EntityMovable,
}
impl Sword {
    pub fn new(x: i32, y: i32) -> Self {
        Sword {
            entity: EntityMovable::new(x, y, Color::YELLOW),
        }
    }
}
impl PowerTrait for Sword {}
impl Movable for Sword {
    fn move_xy(&mut self, x: i32, y: i32) {
        self.entity.move_xy(x, y);
    }
}
impl Draw for Sword {
    fn get_rect(&self) -> Rect {
        self.entity.get_rect()
    }
    fn get_color(&self) -> Color {
        self.entity.get_color()
    }
}
impl Collider for Sword {
    fn get_collider(&self) -> Sphere {
        self.entity.get_collider()
    }
}
