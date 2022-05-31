use crate::power::PowerTrait;
use crate::types::{GetColor, GetDrawingRectangle};
pub trait Canvas {
    fn clear(&mut self);
    fn draw_rectangle<T: GetDrawingRectangle + GetColor>(&mut self, rectangle: &T);
    fn draw_power(&mut self, power: &dyn PowerTrait);
    fn draw_text(&mut self, text: &str);
    fn present(&mut self);
}
