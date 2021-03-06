extern crate sdl2;
use crate::collide::Collider;
use crate::entity::{EntityMovable, Movable};
use crate::power::PowerTrait;
use crate::shapes::Sphere;
use crate::transfo_truc::TransfoTruc;
use crate::types::{Color, GetDrawingRectangle, GetPosition, GetSize, Point, Size};
pub struct Player {
    entity: EntityMovable,
    transfo_trucs: Vec<TransfoTruc>,
    power: Option<Box<dyn PowerTrait>>,
}
impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            entity: EntityMovable::new(x, y, 50, 100, Color::BLUE),
            transfo_trucs: Vec::<TransfoTruc>::new(),
            power: None,
        }
    }
    pub fn add_transfo_truc(&mut self, transfo_truc: TransfoTruc) {
        self.transfo_trucs.push(transfo_truc);
    }
    pub fn get_transfo_trucs_count(&self) -> usize {
        self.transfo_trucs.len()
    }
    pub fn trigger_power<T: PowerTrait>(&mut self) {
        if self.transfo_trucs.len() >= T::get_required_transo_truc() {
            self.power = Some(T::trigger_power(self));
        }
    }
    pub fn get_power(&self) -> &Option<Box<dyn PowerTrait>> {
        &self.power
    }
    pub fn use_power(&mut self) {
        if let Some(power) = &mut self.power {
            power.use_power();
        }
    }
}
impl Movable for Player {
    fn move_xy(&mut self, x: i32, y: i32) {
        self.entity.move_xy(x, y);
        if let Some(power) = &mut self.power {
            power.as_mut().move_xy(x, y);
        }
    }
}
impl GetPosition for Player {
    fn get_position(&self) -> Point {
        self.entity.get_position()
    }
}
impl GetDrawingRectangle for Player {
    fn get_color(&self) -> Color {
        self.entity.get_color()
    }
}
impl GetSize for Player {
    fn get_size(&self) -> Size {
        self.entity.get_size()
    }
}
impl Collider<Sphere> for Player {
    fn get_collider(&self) -> Sphere {
        Sphere::new(
            Point {
                x: self.get_x(),
                y: self.get_y() + self.get_height() as i32 / 2,
            },
            self.get_width() as i32,
        )
    }
}
