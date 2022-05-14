extern crate sdl2;
use sdl2::{rect::Rect, render::Canvas};
use sdl2::video::Window;
pub trait Draw{
    fn get_rect(&self)->Rect;
}

pub fn draw_rectangle<T:Draw>(entity: &T, canvas: &mut Canvas<Window>)->Result<(), String>{
    canvas.fill_rect(entity.get_rect())?;
    Ok(())
}

