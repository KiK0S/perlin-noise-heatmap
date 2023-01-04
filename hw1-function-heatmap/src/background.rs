use crate::draw::*;
use crate::function::PerlinNoise;
use crate::grid::{Dimensions, Grid};
use crate::isoline::Isolines;
use glium::Display;

pub const GRID: Grid = Grid {
    x0: -1.0,
    x1: 1.0,
    y0: -1.0,
    y1: 1.0,
    dimensions: Dimensions { w: 4, h: 4 },
};

pub struct Background {
    pub grid: Grid,
    colors: Vec<(f32, f32, f32)>,
}

pub const BACKGROUND: Background = Background {
    grid: Grid {
        x0: -1.0,
        x1: 1.0,
        y0: -1.0,
        y1: 1.0,
        dimensions: Dimensions { w: 100, h: 100 },
    },
    colors: Vec::new(),
};

impl Background {
    pub fn process(&mut self, function: &PerlinNoise, isolines: &mut Isolines) {
        self.colors
            .reserve(((self.grid.dimensions.w + 1) * (self.grid.dimensions.h + 1)) as usize);
        for (x, y) in self.grid.iterator() {
            let value = function.get_value(x, y, &self.grid);
            self.colors.push((value + 0.4, 0.2, 0.05));
        }

        isolines.process(&self.grid, function);
    }
}

impl Draw for Background {
    fn draw(&mut self, display: &mut Display, target: &mut glium::Frame) {
        let mut shape = Vec::new();
        let mut indices = Vec::new();
        for ((x, y), color) in self.grid.iterator().zip(self.colors.iter()) {
            let size = self.grid.get_cell_width();
            indices.push(0 + shape.len() as u32);
            indices.push(1 + shape.len() as u32);
            indices.push(2 + shape.len() as u32);
            indices.push(0 + shape.len() as u32);
            indices.push(2 + shape.len() as u32);
            indices.push(3 + shape.len() as u32);

            shape.push(ColoredVertex {
                position: [x, y],
                color: [color.0, color.1, color.2],
            });
            shape.push(ColoredVertex {
                position: [x + size, y],
                color: [color.0, color.1, color.2],
            });
            shape.push(ColoredVertex {
                position: [x + size, y + size],
                color: [color.0, color.1, color.2],
            });
            shape.push(ColoredVertex {
                position: [x, y + size],
                color: [color.0, color.1, color.2],
            });
        }
        draw_squares(display, target, shape, indices);
        self.colors.clear();

        // for x in GRID.verticals() {
        //     draw_vertical(display, &mut target, x);
        // }
        // for y in GRID.verticals() {
        //     draw_horizontal(display, &mut target, y);
        // }
        // function.debug_draw(display, &mut target, &BACKGROUND);
    }
}
