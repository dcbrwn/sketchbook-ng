use std::borrow::{BorrowMut};
use std::cell::{RefCell};

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

pub enum GlobalEvent {
    Tick(TickEventData),
    PointerUp(PointerEventData),
    PointerDown(PointerEventData),
    PointerMove(PointerEventData),
    Wheel(WheelEventData),
    WindowResize(WindowResizeData),
}

pub trait EventListener {
    fn dispatch(&mut self, event: &GlobalEvent) -> () {}
}

struct EventBus {
    listeners: Vec<RefCell<Box<dyn EventListener>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            listeners: Vec::new(),
        }
    }

    pub fn attach_listener(&mut self, listener: Box<dyn EventListener>) -> () {
        self.listeners.push(RefCell::new(listener));
    }
}

thread_local! {
    static GLOBAL_BUS: RefCell<EventBus> = RefCell::new(EventBus::new());
}

pub fn attach_global_listener(listener: Box<dyn EventListener>) -> () {
    GLOBAL_BUS.with(|bus| {
        bus.borrow_mut().attach_listener(listener);
    });
}

pub fn dispatch_global_event(event: GlobalEvent) -> () {
    GLOBAL_BUS.with(|wrapped_bus| {
        let bus = wrapped_bus.borrow();

        for mut listener in &bus.listeners {
            listener.borrow_mut().dispatch(&event);
        }
    });
}
