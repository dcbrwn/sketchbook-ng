use crate::common::vec3::Vec3;

pub enum MouseButton {
    Left,
    Middle,
    Right
}

pub trait Sketch {
    fn tick(&self, t: f64) -> () {}
    fn on_click(&self, position: Vec3, button: MouseButton) -> () {}
    fn on_move(&self, p0: Vec3, p1: Vec3, button: MouseButton) -> () {}
}
