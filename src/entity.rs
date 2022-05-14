use crate::collide::{Collider, Point, Sphere};
use crate::draw::Draw;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
#[derive(Clone)]
pub struct EntityBase {
    x: i32,
    y: i32,
    color: Color,
}
impl EntityBase {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        EntityBase { x, y, color }
    }
}
impl Draw for EntityBase {
    fn get_rect(&self) -> Rect {
        return Rect::new(self.x, self.y, 50, 50);
    }
    fn get_color(&self) -> Color {
        self.color
    }
}
impl Collider for EntityBase {
    fn get_collider(&self) -> Sphere {
        Sphere::new(
            Point {
                x: self.x,
                y: self.y,
            },
            50 / 2,
        )
    }
}
#[derive(Clone)]
pub struct EntityMovable {
    entity: EntityBase,
    speed: i32,
}
impl EntityMovable {
    pub fn new(x: i32, y: i32, color: Color) -> Self {
        EntityMovable {
            entity: EntityBase::new(x, y, color),
            speed: 10,
        }
    }
    pub fn move_xy(&mut self, x: i32, y: i32) {
        self.entity.x += x * self.speed;
        self.entity.y += y * self.speed;
    }
}
impl Collider for EntityMovable {
    fn get_collider(&self) -> Sphere {
        self.entity.get_collider()
    }
}
impl Draw for EntityMovable {
    fn get_rect(&self) -> Rect {
        self.entity.get_rect()
    }
    fn get_color(&self) -> Color {
        self.entity.get_color()
    }
}
