use crate::event::Event;
use crate::event::Key;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;

fn to_sdl2key(key: Key) -> Scancode {
    match key {
        Key::Q => Scancode::Q,
        Key::E => Scancode::E,
        Key::Left => Scancode::Left,
        Key::Right => Scancode::Right,
    }
}
impl Event for KeyboardState<'_> {
    fn is_pressed(&self, key: Key) -> bool {
        self.is_scancode_pressed(to_sdl2key(key))
    }
}
