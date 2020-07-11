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

pub struct Initial {
    plotter: Plotter,
    point: usize,
    segment: usize
}

impl Initial {
    pub fn new() -> Self {
        let mut p: Plotter = Plotter::new(get_canvas_by_id("canvas"));

        let point = p.add_primitive(Primitive {
            shape: Shape::Point(Vec3::from_values(100.0, 100.0, 1.0)),
            z_index: 2,
            color: Vec3::from_values(0.0, 0.5, 0.0),
        });

        let segment = p.add_primitive(Primitive {
            shape: Shape::Segment(
                Vec3::from_values(100.0, 100.0, 1.0),
                Vec3::from_values(0.0, 0.0, 1.0)
            ),
            z_index: 1,
            color: Vec3::from_values(1.0, 0.0, 0.0),
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
    fn tick(&self, _t: f64) {
        &self.plotter.render();
    }
}
