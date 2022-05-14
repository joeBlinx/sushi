extern crate sdl2;
use crate::power::PowerTrait;
use sdl2::pixels::Color;
use sdl2::ttf::Font;
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

pub fn draw_rectangle_dyn(
    entity: &dyn PowerTrait,
    canvas: &mut Canvas<Window>,
) -> Result<(), String> {
    canvas.set_draw_color(entity.get_color());
    canvas.fill_rect(entity.get_rect())?;
    Ok(())
}
pub fn clear_canvas(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(130, 130, 130));
    canvas.clear();
}
pub fn display_text(canvas: &mut Canvas<Window>, font: &Font, text: &str) {
    let color = Color::WHITE;
    let surface = font.render(text).solid(color).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = sdl2::render::Texture::from_surface(&surface, &texture_creator).unwrap();
    canvas
        .copy(&texture, None, Rect::new(25, 25, 25, 50))
        .unwrap();
}
