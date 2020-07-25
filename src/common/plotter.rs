use std::cmp::Ordering;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

use super::log::log;
use super::mat3::Mat3;
use super::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Point(Vec3),
    Segment(Vec3, Vec3),
    Ray(Vec3, Vec3),
    Grid(f64),
}

#[derive(Debug, PartialEq)]
pub struct Primitive {
    pub shape: Shape,
    pub z_index: i32,
    pub color: JsValue,
}

#[derive(Debug)]
pub struct Plotter {
    canvas: HtmlCanvasElement,
    canvas_size: (f64, f64),
    context: CanvasRenderingContext2d,
    primitives: Vec<Primitive>,
    draw_order: Vec<usize>,
    transform: Mat3,
    inverse_transform: Mat3,
    clear_color: JsValue,
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
            canvas_size: (0.0, 0.0),
            context,
            primitives: Vec::new(),
            draw_order: Vec::new(),
            transform: Mat3::identity(),
            inverse_transform: Mat3::identity(),
            clear_color: "white".into(),
        }
    }

    pub fn get_transform(&self) -> &Mat3 {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Mat3) {
        let mut inverse_transform = transform.clone();
        inverse_transform.inverse();

        self.transform = transform;
        self.inverse_transform = inverse_transform;
    }

    pub fn set_clear_color(&mut self, color: &Vec3) {
        self.clear_color = color.as_rgb_string().into();
    }

    pub fn update_canvas_size(&mut self) {
        self.canvas_size = (
            self.canvas.width().into(),
            self.canvas.height().into()
        );
    }

    pub fn render(&self) {
        self.clear();

        for primitive_index in &self.draw_order {
            let primitive = &self.primitives[*primitive_index];

            self.context.set_fill_style(&primitive.color);
            self.context.set_stroke_style(&primitive.color);

            match primitive.shape {
                Shape::Point(origin) => self.render_point(&origin),
                Shape::Segment(from, to) => self.render_segment(&from, &to),
                Shape::Grid(step) => self.render_grid(step),
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

    pub fn get_mut(&mut self, primitive_index: usize) -> &mut Primitive {
        &mut self.primitives[primitive_index]
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
        self.context.set_fill_style(&self.clear_color);
        self.context.fill_rect(0.0, 0.0, self.canvas_size.0, self.canvas_size.1);
    }

    pub fn project_to_canvas(&self, point: &Vec3) -> Vec3 {
        &self.transform * point
    }

    pub fn unproject_from_canvas(&self, point: &Vec3) -> Vec3 {
        &self.inverse_transform * point
    }

    fn render_point(&self, origin: &Vec3) {
        let ctx: &CanvasRenderingContext2d = &self.context;

        let target = self.project_to_canvas(origin);

        ctx.begin_path();
        ctx.arc(target.x, target.y, 3.0, 0.0, std::f64::consts::PI * 2.0)
            .unwrap();
        ctx.fill();
    }

    fn line(&self, from: &Vec3, to: &Vec3) {
        let ctx: &CanvasRenderingContext2d = &self.context;

        let actual_from = self.project_to_canvas(from);
        let actual_to = self.project_to_canvas(to);

        ctx.move_to(actual_from.x, actual_from.y);
        ctx.line_to(actual_to.x, actual_to.y);
    }

    fn render_segment(&self, from: &Vec3, to: &Vec3) {
        let ctx: &CanvasRenderingContext2d = &self.context;

        let actual_from = self.project_to_canvas(from);
        let actual_to = self.project_to_canvas(to);

        ctx.begin_path();
        ctx.move_to(actual_from.x, actual_from.y);
        ctx.line_to(actual_to.x, actual_to.y);
        ctx.stroke();
    }

    fn render_grid(&self, step: f64) {
        let ctx: &CanvasRenderingContext2d = &self.context;

        let o = self.unproject_from_canvas(&Vec3 {
            x: self.canvas_size.0 / 2.0, y: self.canvas_size.1 / 2.0, z: 1.0
        }).align(step);

        const OVERSCAN_FACTOR: f64 = 1.5;
        let r = f64::sqrt(o.x * o.x + o.y * o.y);
        let steps = (r * OVERSCAN_FACTOR / step).floor() as i32;

        ctx.begin_path();

        let l = step * (steps as f64) * 2.0;

        for i in -steps..steps {
            let t = (i as f64) * step;
            self.line(
                &Vec3{ x: o.x + t, y: o.y - l, z: 1.0 },
                &Vec3{ x: o.x + t, y: o.y + l, z: 1.0 },
            );

            self.line(
                &Vec3{ x: o.x + l, y: o.y + t, z: 1.0 },
                &Vec3{ x: o.x - l, y: o.y + t, z: 1.0 },
            );
        }

        ctx.stroke();
    }
}
