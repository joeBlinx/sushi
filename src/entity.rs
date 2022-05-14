use crate::draw::Draw;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
pub struct EntityBase {
    x: i32,
    y: i32,
}
impl EntityBase {
    pub fn new(x: i32, y: i32) -> Self {
        EntityBase { x, y }
    }
}
impl Draw for EntityBase {
    fn get_rect(&self) -> Rect {
        return Rect::new(self.x, self.y, 50, 50);
    }
    fn get_color(&self) -> Color {
        Color::RGB(255, 0, 0)
    }
}
pub struct EntityMovable {
    entity: EntityBase,
    speed: i32,
}
impl EntityMovable {
    pub fn new(x: i32, y: i32) -> Self {
        EntityMovable {
            entity: EntityBase::new(x, y),
            speed: 10,
        }
    }
    pub fn move_xy(&mut self, x: i32, y: i32) {
        self.entity.x += x * self.speed;
        self.entity.y += y * self.speed;
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
