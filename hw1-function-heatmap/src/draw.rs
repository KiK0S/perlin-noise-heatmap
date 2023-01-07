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

// pub fn draw_vertical(display: &mut Display, target: &mut Frame, x: f32) {
//     draw_vectors(display, target, &vec![[x, -1.0], [x, 1.0]])
// }

// pub fn draw_horizontal(display: &mut Display, target: &mut Frame, y: f32) {
//     draw_vectors(display, target, &vec![[-1.0, y], [1.0, y]])
// }

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
    display: &mut Display,
    target: &mut Frame,
    vertex_buffer: &glium::VertexBuffer<Vertex>,
    index_buffer: &glium::IndexBuffer<u32>,
) {
    let vertex_shader = r#"
    #version 140
    
    in vec2 position;
    
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "#;
    let fragment_shader = r#"
    #version 140
    
    out vec4 color;
    
    void main() {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    }
    "#;

    let program =
        glium::Program::from_source(display, vertex_shader, fragment_shader, None).unwrap();
    target
        .draw(
            vertex_buffer,
            index_buffer,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
}
