use std::collections::HashMap;

use crate::draw::*;
use crate::function::PerlinNoise;
use crate::grid::Grid;
use glium::Display;
use glium::Frame;

fn solve_by_interpolation(a: f32, b: f32, c: f32, x0: f32, x1: f32) -> f32 {
    // a * t + b (1 - t) = c
    // t = (c - b) / (a - b)
    // res = x * t + b * (1 - t)
    let t = (c - b) / (a - b);
    x0 * t + (1.0 - t) * x1
}

pub struct Isolines {
    c_values: Vec<f32>,
    vectors: Vec<Vertex>,
    indices: Vec<u32>,
    coords_cache: HashMap<(i32, i32, i32), usize>,
}

impl Isolines {
    pub fn new(grid: &Grid, function: &PerlinNoise, cnt: usize) -> Self {
        let mut values: Vec<f32> = grid
            .iterator(false)
            .map(|p| function.get_value(p.0, p.1, grid))
            .collect();
        let len = values.len();
        let cmp = |a: &f32, b: &f32| ((*a * 1000000.0) as i32).cmp(&((*b * 1000000.0) as i32));
        let mut c_values: Vec<f32> = vec![];
        for i in 1..cnt + 1 {
            c_values.push(*values.select_nth_unstable_by(i * len / (cnt + 1), cmp).1);
        }
        Self {
            c_values,
            vectors: Vec::new(),
            indices: Vec::new(),
            coords_cache: HashMap::new(),
        }
    }

    pub fn increase_precision(&mut self, grid: &Grid, function: &PerlinNoise) {
        *self = Self::new(grid, function, self.c_values.len() + 1);
    }

    pub fn decrease_precision(&mut self, grid: &Grid, function: &PerlinNoise) {
        *self = Self::new(grid, function, self.c_values.len().max(1) - 1);
    }

    fn add_point(&mut self, x: f32, y: f32, x_idx: i32, y_idx: i32, tp: i32) -> usize {
        *self
            .coords_cache
            .entry((x_idx, y_idx, tp))
            .or_insert_with(|| {
                self.vectors.push(Vertex { position: [x, y] });
                self.vectors.len() - 1
            })
    }

    fn draw_vector(
        &mut self,
        x_idx: i32,
        y_idx: i32,
        x0: f32,
        y0: f32,
        tp0: i32,
        x1: f32,
        y1: f32,
        tp1: i32,
    ) {
        let idx = self.add_point(x0, y0, x_idx, y_idx, tp0) as u32;
        self.indices.push(idx);
        let idx = self.add_point(x1, y1, x_idx, y_idx, tp1) as u32;
        self.indices.push(idx);
    }

    pub fn process(&mut self, grid: &Grid, function: &PerlinNoise) {
        for c in self.c_values.clone() {
            self.coords_cache.clear();
            for (x, y) in grid.iterator(false) {
                let nx = x + grid.get_cell_width();
                let ny = y + grid.get_cell_height();
                let values: Vec<f32> = vec![(x, y), (nx, y), (x, ny), (nx, ny)]
                    .into_iter()
                    .map(|p| function.get_value(p.0, p.1, grid))
                    .collect();
                let data: Vec<bool> = values.iter().map(|p| *p >= c).collect();
                if data[0] ^ data[1] && data[2] ^ data[3] && data[0] ^ data[2] {
                    let middle_x = x + grid.get_cell_width() / 2.0;
                    let middle_y = y + grid.get_cell_height() / 2.0;
                    let center = function.get_value(middle_x, middle_y, grid) >= c;
                    if center ^ data[0] {
                        let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                        let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                        self.draw_vector(
                            grid.get_point_rev(x, y).0,
                            grid.get_point_rev(x, y).1,
                            mx,
                            y,
                            1,
                            x,
                            my,
                            7,
                        );
                        let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                        let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                        self.draw_vector(
                            grid.get_point_rev(x, y).0,
                            grid.get_point_rev(x, y).1,
                            nx,
                            my,
                            3,
                            mx,
                            ny,
                            5,
                        );
                    } else {
                        let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                        let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                        self.draw_vector(
                            grid.get_point_rev(x, y).0,
                            grid.get_point_rev(x, y).1,
                            mx,
                            y,
                            1,
                            nx,
                            my,
                            3,
                        );
                        let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                        let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                        self.draw_vector(
                            grid.get_point_rev(x, y).0,
                            grid.get_point_rev(x, y).1,
                            x,
                            my,
                            7,
                            mx,
                            ny,
                            5,
                        );
                    }
                    continue;
                }
                if data[0] ^ data[1] && data[2] ^ data[0] {
                    let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                    let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                    self.draw_vector(
                        grid.get_point_rev(x, y).0,
                        grid.get_point_rev(x, y).1,
                        mx,
                        y,
                        1,
                        x,
                        my,
                        7,
                    );
                }
                if data[0] ^ data[1] && data[1] ^ data[3] {
                    let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                    let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                    self.draw_vector(
                        grid.get_point_rev(x, y).0,
                        grid.get_point_rev(x, y).1,
                        mx,
                        y,
                        1,
                        nx,
                        my,
                        3,
                    );
                }
                if data[0] ^ data[2] && data[2] ^ data[3] {
                    let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                    let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                    self.draw_vector(
                        grid.get_point_rev(x, y).0,
                        grid.get_point_rev(x, y).1,
                        x,
                        my,
                        7,
                        mx,
                        ny,
                        5,
                    );
                }
                if data[2] ^ data[3] && data[1] ^ data[3] {
                    let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                    let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                    self.draw_vector(
                        grid.get_point_rev(x, y).0,
                        grid.get_point_rev(x, y).1,
                        nx,
                        my,
                        3,
                        mx,
                        ny,
                        5,
                    );
                }
                if data[0] ^ data[1] && data[2] ^ data[3] && !(data[0] ^ data[2]) {
                    let mx1 = solve_by_interpolation(values[0], values[1], c, x, nx);
                    let mx2 = solve_by_interpolation(values[2], values[3], c, x, nx);
                    self.draw_vector(
                        grid.get_point_rev(x, y).0,
                        grid.get_point_rev(x, y).1,
                        mx1,
                        y,
                        1,
                        mx2,
                        ny,
                        5,
                    );
                }
                if data[0] ^ data[2] && data[1] ^ data[3] && !(data[0] ^ data[1]) {
                    let my1 = solve_by_interpolation(values[0], values[2], c, y, ny);
                    let my2 = solve_by_interpolation(values[1], values[3], c, y, ny);
                    self.draw_vector(
                        grid.get_point_rev(x, y).0,
                        grid.get_point_rev(x, y).1,
                        x,
                        my1,
                        7,
                        nx,
                        my2,
                        3,
                    );
                }
            }
        }
    }
}

impl Draw for Isolines {
    fn draw(&mut self, display: &mut Display, target: &mut Frame) {
        draw_vectors(
            display,
            target,
            &glium::VertexBuffer::new(display, &self.vectors).unwrap(),
            &glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::LinesList,
                &self.indices,
            )
            .unwrap(),
        );
        self.vectors.clear();
        self.indices.clear();
    }
}
