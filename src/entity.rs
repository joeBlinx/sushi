use crate::collide::{Collider, Sphere};
use crate::draw::Draw;
use crate::types::{GetPosition, GetSize, Point, Size};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
pub trait Movable {
    fn move_xy(&mut self, x: i32, y: i32);
}
#[derive(Clone)]
pub struct EntityBase {
    position: Point,
    size: Size,
    color: Color,
}
impl EntityBase {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> Self {
        EntityBase {
            position: Point { x, y },
            size: Size { width, height },
            color,
        }
    }
}
impl GetPosition for EntityBase {
    fn get_position(&self) -> Point {
        self.position.clone()
    }
}
impl GetSize for EntityBase {
    fn get_size(&self) -> Size {
        self.size.clone()
    }
}

impl Draw for EntityBase {
    fn get_rect(&self) -> Rect {
        return Rect::new(
            self.get_x(),
            self.get_y(),
            self.get_width(),
            self.get_height(),
        );
    }
    fn get_color(&self) -> Color {
        self.color
    }
}
impl Collider for EntityBase {
    fn get_collider(&self) -> Sphere {
        Sphere::new(
            Point {
                x: self.get_x(),
                y: self.get_y(),
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
    fn get_size(&self) -> Size {
        self.entity.get_size()
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
        self.entity.position.x += x * self.speed;
        self.entity.position.y += y * self.speed;
    }
}
impl GetPosition for EntityMovable {
    fn get_position(&self) -> Point {
        self.entity.get_position()
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
