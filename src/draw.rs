extern crate sdl2;
use crate::canvas::Canvas;
use crate::power::PowerTrait;
use crate::types;
use crate::types::{DrawingRectangle, GetDrawingRectangle};
use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::{rect::Rect, render};
fn to_sdl_rect(rectangle: DrawingRectangle) -> Rect {
    Rect::new(
        rectangle.upper_left.x,
        rectangle.upper_left.y,
        rectangle.size.width,
        rectangle.size.height,
    )
}
fn to_sdl_color(color: types::Color) -> Color {
    Color {
        r: color.red,
        g: color.green,
        b: color.blue,
        a: color.alpha,
    }
}

pub fn draw_rectangle<T: GetDrawingRectangle>(
    entity: &T,
    canvas: &mut render::Canvas<Window>,
) -> Result<(), String> {
    canvas.set_draw_color(to_sdl_color(entity.get_color()));
    canvas.fill_rect(to_sdl_rect(entity.get_drawing_rectangle()))?;
    Ok(())
}

pub fn draw_rectangle_dyn(
    entity: &dyn PowerTrait,
    canvas: &mut render::Canvas<Window>,
) -> Result<(), String> {
    canvas.set_draw_color(to_sdl_color(entity.get_color()));
    canvas.fill_rect(to_sdl_rect(entity.get_drawing_rectangle()))?;
    Ok(())
}
pub fn clear_canvas(canvas: &mut render::Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(130, 130, 130));
    canvas.clear();
}
pub fn display_text(canvas: &mut render::Canvas<Window>, font: &Font, text: &str) {
    let color = Color::WHITE;
    let surface = font.render(text).solid(color).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = sdl2::render::Texture::from_surface(&surface, &texture_creator).unwrap();
    canvas
        .copy(&texture, None, Rect::new(25, 25, 25, 50))
        .unwrap();
}
pub struct CanvasFont<'a> {
    pub canvas: &'a mut render::Canvas<Window>,
    pub font: &'a Font<'a, 'a>,
}
impl<'a> Canvas for CanvasFont<'a> {
    fn clear(&mut self) {
        clear_canvas(self.canvas);
    }
    fn draw_rectangle<T: GetDrawingRectangle>(&mut self, rectangle: &T) {
        draw_rectangle(rectangle, self.canvas).unwrap();
    }
    fn draw_text(&mut self, text: &str) {
        display_text(self.canvas, self.font, text);
    }
    fn present(&mut self) {
        self.canvas.present();
    }
    fn draw_power(&mut self, power: &dyn PowerTrait) {
        draw_rectangle_dyn(power, self.canvas).unwrap();
    }
}
