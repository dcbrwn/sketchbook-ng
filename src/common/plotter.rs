use std::cmp::Ordering;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

use super::log::log;
use super::mat3::Mat3;
use super::vec3::Vec3;

#[derive(Debug, PartialEq, Eq)]
pub enum Shape {
    Point(Vec3),
    Segment(Vec3, Vec3),
    Ray(Vec3, Vec3),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Primitive {
    pub shape: Shape,
    pub z_index: i32,
    pub color: Vec3,
}

#[derive(Debug)]
pub struct Plotter {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    primitives: Vec<Primitive>,
    draw_order: Vec<usize>,
    transform: Mat3,
}

impl Plotter {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Plotter {
            canvas,
            context,
            primitives: Vec::new(),
            draw_order: Vec::new(),
            transform: Mat3::identity(),
        }
    }

    pub fn render(&self) {
        self.clear();

        for primitive_index in &self.draw_order {
            let primitive = &self.primitives[*primitive_index];
            let color_string = &primitive.color.as_rgb_string();

            // log(color_string);
            self.context.set_fill_style(&color_string.into());
            self.context.set_stroke_style(&color_string.into());

            match primitive.shape {
                Shape::Point(origin) => self.render_point(&origin),
                Shape::Segment(from, to) => self.render_segment(&from, &to),
                _ => (),
            }
        }
    }

    pub fn add_primitive(&mut self, primitive: Primitive) -> usize {
        let last_index = self.primitives.len();
        self.primitives.push(primitive);
        self.calc_draw_order();
        last_index
    }

    fn calc_draw_order(&mut self) {
        let len = self.primitives.len();
        self.draw_order.resize(len, 0);

        for i in 0 .. len {
            self.draw_order[i] = i;
        }

        let primitives = &self.primitives;

        self.draw_order.sort_by(|&a, &b| -> Ordering {
            let z_a = primitives[a].z_index;
            let z_b = primitives[b].z_index;
            z_a.cmp(&z_b)
        });
    }

    fn clear(&self) {
        self.context.set_fill_style(&"white".into());
        self.context.fill_rect(0.0, 0.0, self.canvas.width().into(), self.canvas.height().into());
    }

    fn map_point_to_canvas(&self, point: &Vec3) -> Vec3 {
        &self.transform * point
    }

    fn render_point(&self, origin: &Vec3) {
        let ctx: &CanvasRenderingContext2d = &self.context;

        let target = self.map_point_to_canvas(origin);

        ctx.begin_path();
        ctx.arc(target.x, target.y, 3.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        ctx.fill();
    }

    fn render_segment(&self, from: &Vec3, to: &Vec3) {
        let ctx: &CanvasRenderingContext2d = &self.context;

        let actual_from = self.map_point_to_canvas(from);
        let actual_to = self.map_point_to_canvas(to);

        ctx.begin_path();
        ctx.move_to(actual_from.x, actual_from.y);
        ctx.line_to(actual_to.x, actual_to.y);
        ctx.stroke();
    }
}
