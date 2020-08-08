use std::f64;

use crate::interop::log::*;
use crate::plotter::{
    Plotter,
    Primitive,
    Shape,
};
use crate::math::vec3::Vec3;
use crate::interop::events::*;
use crate::interop::events::GlobalEvent::*;
use crate::vec3;

use crate::common::colors::*;

pub struct Initial {
    plotter: Plotter,
    point: usize,
    segment: usize
}

impl Initial {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Self {
        let mut p: Plotter = Plotter::new(canvas);

        p.set_transform(
            p.get_transform()
                .translate(&vec3!(120.0, 130.0, 1.0))
                .rotate(f64::consts::PI / 2.5)
        );

        p.set_clear_color(&PAPER);

        p.add_primitive(Primitive {
            shape: Shape::Grid(50.0),
            z_index: 0,
            color: LIGHT_BLUE_INK.as_rgb_string().into(),
        });

        p.add_primitive(Primitive {
            shape: Shape::Grid(250.0),
            z_index: 1,
            color: BLUE_INK.as_rgb_string().into(),
        });

        // p.set_transform(p.get_transform().translate(&Vec3 { x: 10.0, y: 20.0, z: 1.0 }));
        let point = p.add_primitive(Primitive {
            shape: Shape::Point(Vec3::from_values(100.0, 100.0, 1.0)),
            z_index: 2,
            color: RED.as_rgb_string().into(),
        });

        let segment = p.add_primitive(Primitive {
            shape: Shape::Segment(
                Vec3::from_values(100.0, 100.0, 1.0),
                Vec3::from_values(0.0, 0.0, 1.0)
            ),
            z_index: 1,
            color: GREEN.as_rgb_string().into(),
        });

        let result: Initial = Initial {
            plotter: p,
            point,
            segment,
        };

        return result
    }

    fn on_tick(&mut self, data: &TickEventData) {
        self.plotter.update_canvas_size();

        let point = self.plotter.get_mut(self.point);

        if let Shape::Point(ref mut pos) = point.shape {
            (*pos).x = 100.0 + f64::sin(data.time) * 100.0;
        }

        self.plotter.render();
    }

    fn on_pointer_down(&mut self, data: &PointerEventData) {
        let p = self.plotter.project_to_canvas(&vec3!(data.x, data.y, 1.0));

        self.plotter.add_primitive(Primitive {
            shape: Shape::Point(p),
            z_index: 2,
            color: RED.as_rgb_string().into(),
        });
    }
}

impl EventListener for Initial {
    fn dispatch(&mut self, event: &GlobalEvent) {
        match event {
            Tick(ref data) => self.on_tick(data),
            PointerDown(ref data) => self.on_pointer_down(data),
            _ => ()
        }
    }
}
