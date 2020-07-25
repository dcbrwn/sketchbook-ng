use crate::common::vec3::Vec3;

pub struct TickEventData {
    pub time: f64,
}

pub struct PointerEventData {
    pub x: f64,
    pub y: f64,
    pub button: i8,
}

pub struct WheelEventData {
    pub dx: f64,
    pub dy: f64,
    pub px: f64,
    pub py: f64,
}

pub struct WindowResizeData {
    pub width: f64,
    pub height: f64,
}

pub enum SketchEvent {
    Tick(TickEventData),
    PointerUp(PointerEventData),
    PointerDown(PointerEventData),
    PointerMove(PointerEventData),
    Wheel(WheelEventData),
    WindowResize(WindowResizeData),
}

pub trait EventTarget {
    fn dispatch(&mut self, event: SketchEvent) -> () {}
}
