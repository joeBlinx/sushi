extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::{rect::Rect, render::Canvas};
pub trait Draw {
    fn get_rect(&self) -> Rect;
    fn get_color(&self) -> Color;
}

pub fn draw_rectangle<T: Draw>(entity: &T, canvas: &mut Canvas<Window>) -> Result<(), String> {
    canvas.set_draw_color(entity.get_color());
    canvas.fill_rect(entity.get_rect())?;
    Ok(())
}

pub fn clear_canvas(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(130, 130, 130));
    canvas.clear();
}
