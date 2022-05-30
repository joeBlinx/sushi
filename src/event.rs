pub trait Event {
    fn is_pressed(&self, key: Key) -> bool;
}
pub enum Key {
    Right,
    Left,
    E,
    Q,
}
