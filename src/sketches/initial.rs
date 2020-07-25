use std::f64;

use crate::common::log::*;
use crate::common::dom::{get_canvas_by_id};
use super::sketch::*;
use crate::common::vec3::Vec3;
use crate::common::plotter::{
    Shape,
    Primitive,
    Plotter,
};

use super::colors::*;

pub struct Initial {
    plotter: Plotter,
    point: usize,
    segment: usize
}

impl Initial {
    pub fn new() -> Self {
        let mut p: Plotter = Plotter::new(get_canvas_by_id("canvas"));

        p.set_transform(
            p.get_transform()
                .translate(&Vec3 { x: 100.0, y: 100.0, z: 1.0 })
                .rotate(f64::consts::PI / 4.0)
        );

        p.set_clear_color(&PAPER);

        p.add_primitive(Primitive {
            shape: Shape::Grid(50.0),
            z_index: 0,
            color: LIGHT_BLUE_INK.as_rgb_string().into(),
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

        log(&format!("Plotter {:?}", p));

        let result: Initial = Initial {
            plotter: p,
            point: point,
            segment: segment,
        };

        return result
    }
}

impl Sketch for Initial {
    fn tick(&mut self, t: f64) {
        self.plotter.update_canvas_size();

        let point = self.plotter.get_mut(self.point);

        if let Shape::Point(ref mut pos) = point.shape {
            (*pos).x = 100.0 + f64::sin(t) * 100.0;
        }

        self.plotter.render();
    }
}
