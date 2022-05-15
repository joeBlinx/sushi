use crate::collide::{Collider, Sphere};
use crate::draw::Draw;
use crate::entity::EntityBase;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Clone)]
pub struct TransfoTruc {
    entity: EntityBase,
}
impl TransfoTruc {
    pub fn new(x: i32, y: i32) -> Self {
        TransfoTruc {
            entity: EntityBase::new(x, y, 50, 50, Color::GREEN),
        }
    }
}
impl Draw for TransfoTruc {
    fn get_rect(&self) -> Rect {
        self.entity.get_rect()
    }
    fn get_color(&self) -> Color {
        self.entity.get_color()
    }
}
impl Collider for TransfoTruc {
    fn get_collider(&self) -> Sphere {
        self.entity.get_collider()
    }
}
