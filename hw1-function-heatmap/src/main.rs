#![deny(clippy::correctness)]
#![deny(clippy::perf)]
// #![allow(clippy::all)]
//
use std::f32::consts::PI;
pub mod background;
pub mod draw;
pub mod function;
pub mod grid;
pub mod isoline;
use crate::isoline::Isolines;
use background::draw_background;
use function::PerlinNoise;
use glium::glutin::event::{ElementState, VirtualKeyCode, WindowEvent};

/// Example from https://glium-doc.github.io/#/tuto-01-getting-started
/// with some tweaks so color changes smoothly

fn main() {
    // 1. The **winit::EventsLoop** for handling events.
    let events_loop = glium::glutin::event_loop::EventLoop::new();
    // 2. Parameters for building the Window.
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1000.0, 1000.0))
        .with_title("Perlin Noise function");
    // 3. Parameters for building the OpenGL context.
    let cb = glium::glutin::ContextBuilder::new();
    // 4. Build the Display with the given window and OpenGL context parameters and register the
    //    window with the events_loop.
    let mut display = glium::Display::new(wb, cb, &events_loop).unwrap();
    let mut cnt = 0.0;
    let mut function = PerlinNoise::new(background::GRID.dimensions);
    let mut isolines_cnt = 5;
    let mut isolines = Isolines::new(&background::BACKGROUND, &function, isolines_cnt);
    events_loop.run(move |ev, _, control_flow| {
        draw_background(&mut display, cnt, &function, &isolines);
        function.update();
        cnt += PI / 2000.0;

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        if let glium::glutin::event::Event::WindowEvent { event, .. } = ev {
            match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    if let ElementState::Pressed = input.state {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Plus) => {
                                isolines_cnt += 1;
                                isolines =
                                    Isolines::new(&background::BACKGROUND, &function, isolines_cnt);
                            }
                            Some(VirtualKeyCode::Minus) => {
                                isolines_cnt -= 1;
                                isolines = Isolines::new(&background::BACKGROUND, &function, isolines_cnt);
                            }
                            _ => (),
                        }
                    }
                },
                _ => (),
            }
        }
    });
}
