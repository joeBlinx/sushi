#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub trait GetPosition {
    fn get_position(&self) -> Point;
    fn get_x(&self) -> i32 {
        self.get_position().x
    }
    fn get_y(&self) -> i32 {
        self.get_position().y
    }
}
#[derive(Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
pub trait GetSize {
    fn get_size(&self) -> Size;
    fn get_width(&self) -> u32 {
        self.get_size().width
    }
    fn get_height(&self) -> u32 {
        self.get_size().height
    }
}
#[derive(Clone)]
pub struct DrawingRectangle {
    pub upper_left: Point,
    pub size: Size,
}
pub trait GetDrawingRectangle: GetSize + GetPosition {
    fn get_drawing_rectangle(&self) -> DrawingRectangle {
        DrawingRectangle {
            upper_left: self.get_position(),
            size: self.get_size(),
        }
    }
}
