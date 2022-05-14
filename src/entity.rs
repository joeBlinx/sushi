use crate::draw::Draw;
use sdl2::rect::Rect;
pub struct EntityBase{
}
impl Draw for EntityBase{
    fn get_rect(&self) -> Rect{
        return Rect::new(0, 0, 50 ,50);
    }
}
