use crate::collide::{Collider, Point, Sphere};
use crate::draw::Draw;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
pub trait GetPosition {
    fn get_x(&self) -> i32;
    fn get_y(&self) -> i32;
}
pub trait GetSize {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
pub trait Movable {
    fn move_xy(&mut self, x: i32, y: i32);
}
#[derive(Clone)]
pub struct EntityBase {
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    color: Color,
}
impl EntityBase {
    pub fn new(x: i32, y: i32, w: u32, h: u32, color: Color) -> Self {
        EntityBase { x, y, w, h, color }
    }
}
impl GetSize for EntityBase {
    fn width(&self) -> u32 {
        self.w
    }
    fn height(&self) -> u32 {
        self.h
    }
}
impl Draw for EntityBase {
    fn get_rect(&self) -> Rect {
        return Rect::new(self.x, self.y, self.w, self.h);
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
impl GetSize for EntityMovable {
    fn width(&self) -> u32 {
        self.entity.width()
    }
    fn height(&self) -> u32 {
        self.entity.height()
    }
}
impl EntityMovable {
    pub fn new(x: i32, y: i32, w: u32, h: u32, color: Color) -> Self {
        EntityMovable {
            entity: EntityBase::new(x, y, w, h, color),
            speed: 10,
        }
    }
}
impl Movable for EntityMovable {
    fn move_xy(&mut self, x: i32, y: i32) {
        self.entity.x += x * self.speed;
        self.entity.y += y * self.speed;
    }
}
impl GetPosition for EntityMovable {
    fn get_x(&self) -> i32 {
        self.entity.x
    }
    fn get_y(&self) -> i32 {
        self.entity.y
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
