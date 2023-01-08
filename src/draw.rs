use glium::uniforms::EmptyUniforms;
use glium::Display;
use glium::Frame;
use glium::Surface;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct Color {
    pub color: [f32; 3],
}

glium::implement_vertex!(Vertex, position);
glium::implement_vertex!(Color, color);
glium::implement_vertex!(ColoredVertex, position, color);

pub trait Draw {
    fn draw(&mut self, display: &mut Display, target: &mut Frame);
}

pub fn draw_squares(
    target: &mut Frame,
    vertex_buffer: &glium::VertexBuffer<Vertex>,
    color_buffer: &glium::VertexBuffer<Color>,
    index_buffer: &glium::IndexBuffer<u32>,
    program: &glium::Program,
) {
    target
        .draw(
            (vertex_buffer, color_buffer),
            index_buffer,
            program,
            &EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
}

pub fn draw_vectors(
    target: &mut Frame,
    vertex_buffer: &glium::VertexBuffer<Vertex>,
    index_buffer: &glium::IndexBuffer<u32>,
    program: &glium::Program,
) {
    target
        .draw(
            vertex_buffer,
            index_buffer,
            program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
}
