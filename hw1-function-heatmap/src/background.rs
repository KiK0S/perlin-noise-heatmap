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
    colors: Vec<Color>,
    vertices: glium::VertexBuffer<Vertex>,
    indices: glium::IndexBuffer<u32>,
    program: glium::Program,
}

impl Background {
    pub fn new(grid: Grid, display: &Display) -> Self {
        let mut shape = Vec::new();
        let mut indices = Vec::new();
        for (x, y) in grid.iterator(true) {
            shape.push(Vertex { position: [x, y] });
        }
        assert!(shape.len() == ((grid.dimensions.h + 1) * (grid.dimensions.w + 1)) as usize);
        for idx in 0..shape.len() {
            if idx as i32 / (grid.dimensions.h + 1) == grid.dimensions.w {
                continue;
            }
            if idx as i32 % (grid.dimensions.h + 1) == grid.dimensions.h {
                continue;
            }
            indices.push(idx as u32);
            indices.push((idx + 1 + grid.dimensions.h as usize) as u32);
            indices.push((idx + 2 + grid.dimensions.h as usize) as u32);
            indices.push(idx as u32);
            indices.push((idx + 2 + grid.dimensions.h as usize) as u32);
            indices.push((idx + 1_usize) as u32);
        }
        let vertex_shader = r#"
        #version 330
        
        in vec2 position;
        in vec3 color;
        out vec4 fragColor;
        
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
            fragColor.rgb = color;
            fragColor.a = 1;
        }
        "#;
        let fragment_shader = r#"
        #version 330
        
        in vec4 fragColor;
        out vec4 color;
        
        void main() {
            color = fragColor;
        }
        "#;

        let program =
            glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap();

        Background {
            grid,
            colors: Vec::new(),
            vertices: glium::VertexBuffer::new(display, &shape).unwrap(),
            indices: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &indices,
            )
            .unwrap(),
            program,
        }
    }

    pub fn process(&mut self, function: &PerlinNoise, isolines: &mut Isolines) {
        self.colors
            .reserve(((self.grid.dimensions.w + 1) * (self.grid.dimensions.h + 1)) as usize);
        for (x, y) in self.grid.iterator(true) {
            let value = function.get_value(x, y, &self.grid);
            self.colors.push(Color {
                color: [value + 0.4, 0.2, 0.05],
            });
        }

        isolines.process(&self.grid, function);
    }
}

impl Draw for Background {
    fn draw(&mut self, display: &mut Display, target: &mut glium::Frame) {
        draw_squares(
            target,
            &self.vertices,
            &glium::VertexBuffer::new(display, &self.colors).unwrap(),
            &self.indices,
            &self.program,
        );
        self.colors.clear();
    }
}
