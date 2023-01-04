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
    vectors_cache: Vec<[f32; 2]>,
}

impl Isolines {
    pub fn new(grid: &Grid, function: &PerlinNoise, cnt: usize) -> Self {
        let mut values: Vec<f32> = grid
            .iterator()
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
            vectors_cache: Vec::new(),
        }
    }

    pub fn increase_precision(&mut self, grid: &Grid, function: &PerlinNoise) {
        *self = Self::new(grid, function, self.c_values.len() + 1);
    }

    pub fn decrease_precision(&mut self, grid: &Grid, function: &PerlinNoise) {
        *self = Self::new(grid, function, self.c_values.len().max(1) - 1);
    }

    fn draw_vector(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        self.vectors_cache.push([x0, y0]);
        self.vectors_cache.push([x1, y1]);
    }

    pub fn process(&mut self, grid: &Grid, function: &PerlinNoise) {
        self.vectors_cache
            .reserve(2 * ((grid.dimensions.w + 1) * (grid.dimensions.h + 1)) as usize);

        for c in self.c_values.clone() {
            for (x, y) in grid.iterator() {
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
                    let center = function.get_value(middle_x, middle_y, grid) >= x;
                    if center ^ data[0] {
                        let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                        let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                        self.draw_vector(mx, y, x, my);
                        let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                        let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                        self.draw_vector(nx, my, mx, ny);
                    } else {
                        let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                        let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                        self.draw_vector(mx, y, nx, my);
                        let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                        let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                        self.draw_vector(x, my, mx, ny);        
                    }
                    continue;
                }
                if data[0] ^ data[1] && data[2] ^ data[0] {
                    let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                    let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                    self.draw_vector(mx, y, x, my);
                }
                if data[0] ^ data[1] && data[1] ^ data[3] {
                    let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                    let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                    self.draw_vector(mx, y, nx, my);
                }
                if data[0] ^ data[2] && data[2] ^ data[3] {
                    let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                    let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                    self.draw_vector(x, my, mx, ny);
                }
                if data[2] ^ data[3] && data[1] ^ data[3] {
                    let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                    let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                    self.draw_vector(nx, my, mx, ny);
                }
                if data[0] ^ data[1] && data[2] ^ data[3] && !(data[0] ^ data[2]) {
                    let mx1 = solve_by_interpolation(values[0], values[1], c, x, nx);
                    let mx2 = solve_by_interpolation(values[2], values[3], c, x, nx);
                    self.draw_vector(mx1, y, mx2, ny);
                }
                if data[0] ^ data[2] && data[1] ^ data[3] && !(data[0] ^ data[1]) {
                    let my1 = solve_by_interpolation(values[0], values[2], c, y, ny);
                    let my2 = solve_by_interpolation(values[1], values[3], c, y, ny);
                    self.draw_vector(x, my1, nx, my2);
                }
            }
        }
    }
}

impl Draw for Isolines {
    fn draw(&mut self, display: &mut Display, target: &mut Frame) {
        draw_vectors(display, target, &self.vectors_cache);
        self.vectors_cache.clear();
    }
}
