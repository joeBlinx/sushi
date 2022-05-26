use crate::collide::{Collider, Sphere};
use crate::draw::Draw;
use crate::entity::EntityBase;
use crate::types::{GetDrawingRectangle, GetPosition, GetSize, Point, Size};
use sdl2::pixels::Color;

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
impl GetSize for TransfoTruc {
    fn get_size(&self) -> Size {
        self.entity.get_size()
    }
}
impl GetPosition for TransfoTruc {
    fn get_position(&self) -> Point {
        self.entity.get_position()
    }
}
impl GetDrawingRectangle for TransfoTruc {}
impl Draw for TransfoTruc {
    fn get_color(&self) -> Color {
        self.entity.get_color()
    }
}
impl Collider for TransfoTruc {
    fn get_collider(&self) -> Sphere {
        self.entity.get_collider()
    }
}
