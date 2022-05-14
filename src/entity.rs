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
    pub fn move_xy(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
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
