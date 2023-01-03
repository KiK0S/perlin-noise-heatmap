use std::f32::consts::PI;
pub mod background;
pub mod function;
pub mod grid;
pub mod draw;
pub mod isoline;
use background::draw_background;
use crate::isoline::Isolines;
use function::PerlinNoise;
use glium;

/// Example from https://glium-doc.github.io/#/tuto-01-getting-started
/// with some tweaks so color changes smoothly

fn main() {
    // 1. The **winit::EventsLoop** for handling events.
    let events_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1000.0, 1000.0))
        .with_title("Hello world");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let mut display = glium::Display::new(wb, cb, &events_loop).unwrap();
    let mut cnt = 0.0;
    let mut function = PerlinNoise::new(background::GRID.dimensions);
    let isolines = Isolines::new(&background::BACKGROUND, &function, 5);
    events_loop.run(move |ev, _, control_flow| {
        draw_background(&mut display, cnt, &function, &isolines);
        function.update();
        cnt += PI / 2000.0;
        
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
