use std::f32::consts::PI;

use crate::grid::{Grid, Dimensions};
use crate::draw::draw_vector;
use glium::Display;
use glium::Frame;
use rand::Rng;

#[derive(Debug)]
struct RotatingVector {
    angle: f32,
    rotation_speed: f32,
}

pub struct PerlinNoise {
    vectors: Vec<RotatingVector>,
    dimensions: Dimensions,
}

impl PerlinNoise {
    pub fn new(dimensions: Dimensions) -> Self {
        let mut vectors = Vec::new();
        for _ in 0..(dimensions.w + 1) * (dimensions.h + 1) {
            vectors.push(RotatingVector{
                angle: rand::thread_rng().gen_range(0.0..2.0 * PI),
                rotation_speed: 0.1,
            });
        }
        Self {
            vectors,
            dimensions,
        }
    }

    fn _normalize(p: &mut (f32, f32)) {
        let len = (p.0 * p.0 + p.1 * p.1).sqrt();
        p.0 /= len;
        p.1 /= len;
    }

    fn dot_product(p: (f32, f32), q: (f32, f32)) -> f32 {
        p.0 * q.0 + p.1 * q.1
    }


    fn interpolate(a: f32, b: f32, x: f32) -> f32 {
        assert!(0.0 <= x && x <= 1.0);
        // return (b - a) * (x) + a;
        return (b - a) * (3.0 * x.powf(2.0) - 2.0 * x.powf(3.0)) + a;
    }

    fn map_idx(&self, x: i32, y: i32) -> i32 {
        x * self.dimensions.h + y
    }

    fn _map_idx_reverse(&self, idx: i32) -> (i32, i32) {
        (idx / self.dimensions.h, idx % self.dimensions.h)
    }

    pub fn update(&mut self) {
        for mut vector in &mut self.vectors {
            vector.angle += vector.rotation_speed;
        }
    }

    pub fn get_value(&self, x: f32, y: f32, grid: &Grid) -> f32 {
        let xl = ((x - grid.x0 - 0.0001) / ((grid.x1 - grid.x0) / self.dimensions.w as f32)) as i32;
        let yl = ((y - grid.y0 - 0.0001) / ((grid.y1 - grid.y0) / self.dimensions.h as f32)) as i32;
        let mut deltas  = vec![];
        for coords in [(xl, yl), (xl + 1, yl), (xl, yl + 1), (xl + 1, yl + 1)] {
            let vector = &self.vectors[self.map_idx(coords.0, coords.1) as usize];
            let point = grid.get_point(coords.0 * grid.dimensions.w / self.dimensions.w, coords.1 * grid.dimensions.h / self.dimensions.h);
            let p = (x - point.0, y - point.1);
            let q = (vector.angle.cos() * 0.5 * ((grid.x1 - grid.x0) / self.dimensions.w as f32), vector.angle.sin() * 0.5 * ((grid.y1 - grid.y0) / self.dimensions.h as f32));
            deltas.push(Self::dot_product(p, q));
        }
        let pl = grid.get_point(xl * grid.dimensions.w / self.dimensions.w, yl * grid.dimensions.h / self.dimensions.h);
        let dx = (x - pl.0) / ((grid.x1 - grid.x0) / self.dimensions.w as f32);
        let dy = (y - pl.1) / ((grid.y1 - grid.y0) / self.dimensions.h as f32);
        10.0 * Self::interpolate(Self::interpolate(deltas[0], deltas[1], dx),Self::interpolate(deltas[2], deltas[3], dx), dy)
    }

    pub fn debug_draw(&self, display: &mut Display, target: &mut Frame, grid: &Grid) {
        for x in 0..self.dimensions.w+1 {
            for y in 0..self.dimensions.h+1 {
                let p = grid.get_point(x * grid.dimensions.w / self.dimensions.w, y * grid.dimensions.h / self.dimensions.h);
                let angle = self.vectors[self.map_idx(x, y) as usize].angle;
                draw_vector(display, target, p.0, p.1, p.0 + angle.cos() / 5.0, p.1 + angle.sin() / 5.0);
            }
        }
    }
}
