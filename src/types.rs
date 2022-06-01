#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
#[derive(Clone)]
pub struct DrawingRectangle {
    pub upper_left: Point,
    pub size: Size,
}
#[derive(Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
impl Color {
    pub const BLUE: Color = Color {
        red: 0,
        green: 0,
        blue: 255,
        alpha: 255,
    };
    pub const GREEN: Color = Color {
        red: 0,
        green: 255,
        blue: 0,
        alpha: 255,
    };
    pub const YELLOW: Color = Color {
        red: 255,
        green: 255,
        blue: 0,
        alpha: 255,
    };
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
pub trait GetSize {
    fn get_size(&self) -> Size;
    fn get_width(&self) -> u32 {
        self.get_size().width
    }
    fn get_height(&self) -> u32 {
        self.get_size().height
    }
}
pub trait GetDrawingRectangle: GetSize + GetPosition {
    fn get_drawing_rectangle(&self) -> DrawingRectangle {
        DrawingRectangle {
            upper_left: self.get_position(),
            size: self.get_size(),
        }
    }
    fn get_color(&self) -> Color;
}
