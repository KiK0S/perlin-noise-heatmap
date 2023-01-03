use glium::Display;
use glium::Frame;
use crate::grid::Grid;
use crate::function::PerlinNoise;
use crate::draw::*;

fn solve_by_interpolation(a: f32, b: f32, c: f32, x0: f32, x1: f32) -> f32 {
    // a * t + b (1 - t) = c
    // t = (c - b) / (a - b)
    // res = x * t + b * (1 - t)
    let t = (c - b) / (a - b);
    x0 * t + (1.0 - t) * x1
}

pub struct Isolines {
    c_values: Vec<f32>,
}

impl Isolines {

    pub fn new(grid: &Grid, function: &PerlinNoise, cnt: usize) -> Self {
        let mut values: Vec<f32> = grid.iterator().map(|p| function.get_value(p.0, p.1, grid)).collect();
        let len = values.len();
        let cmp = |a: &f32, b: &f32| {
            ((*a * 1000000.0) as i32).cmp(&((*b * 1000000.0) as i32))
        };
        let mut c_values: Vec<f32> = vec![];
        for i in 1..cnt {
            c_values.push(*values.select_nth_unstable_by(i * len / cnt, cmp).1);
        }
        Self {
            c_values
        }
    }

    pub fn draw(&self, display: &mut Display, target: &mut Frame, grid: &Grid, function: &PerlinNoise) {
        for c in self.c_values.iter() {
            let c = *c;
            for (x, y) in grid.iterator() {
                let nx = x + grid.get_cell_width();
                let ny = y + grid.get_cell_height();
                let values: Vec<f32> = vec![(x, y), (nx, y), (x, ny), (nx, ny)].into_iter().map(|p| function.get_value(p.0, p.1, &grid)).collect();
                let data: Vec<bool> = values.iter().map(|p| *p >= c).collect();
                if data[0] ^ data[1] && data[2] ^ data[0] {
                    let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                    let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                    draw_vector(display, target, mx, y, x, my);
                }
                if data[0] ^ data[1] && data[1] ^ data[3] {
                    let mx = solve_by_interpolation(values[0], values[1], c, x, nx);
                    let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                    draw_vector(display, target, mx, y, nx, my);
                }
                if data[0] ^ data[2] && data[2] ^ data[3] {
                    let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                    let my = solve_by_interpolation(values[0], values[2], c, y, ny);
                    draw_vector(display, target, x, my, mx, ny);
                }
                if data[2] ^ data[3] && data[1] ^ data[3] {
                    let mx = solve_by_interpolation(values[2], values[3], c, x, nx);
                    let my = solve_by_interpolation(values[1], values[3], c, y, ny);
                    draw_vector(display, target, nx, my, mx, ny);
                }
                if data[0] ^ data[1] && data[2] ^ data[3] && !(data[0] ^ data[2]) {
                    let mx1 = solve_by_interpolation(values[0], values[1], c, x, nx);
                    let mx2 = solve_by_interpolation(values[2], values[3], c, x, nx);
                    draw_vector(display, target, mx1, y, mx2, ny);
                }
                if data[0] ^ data[2] && data[1] ^ data[3] && !(data[0] ^ data[1]) {
                    let my1 = solve_by_interpolation(values[0], values[2], c, y, ny);
                    let my2 = solve_by_interpolation(values[1], values[3], c, y, ny);
                    draw_vector(display, target, x, my1, nx, my2);
                }
                
            }
        }
    }

}
