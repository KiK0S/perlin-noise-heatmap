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
use draw::Draw;
use function::PerlinNoise;
use glium::glutin::event::{ElementState, VirtualKeyCode, WindowEvent};
use glium::Surface;

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
    let mut background = background::BACKGROUND;
    let mut function = PerlinNoise::new(background::GRID.dimensions);
    let mut isolines = Isolines::new(&background.grid, &function, 5);
    let mut last_time = std::time::Instant::now();
    let mut paused: bool = false;
    events_loop.run(move |ev, _, control_flow| {
        let cur_time = std::time::Instant::now();

        if (cur_time - last_time).as_millis() > 100 && !paused {
            // redraw

            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_millis(100);
            *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

            let mut target = display.draw();
            target.clear_color(1.0, 1.0, 1.0, 1.0);
            function.update();
            background.process(&function, &mut isolines);
            background.draw(&mut display, &mut target);
            isolines.draw(&mut display, &mut target);
            target.finish().unwrap();
            last_time = std::time::Instant::now();
        }

        if let glium::glutin::event::Event::WindowEvent { event, .. } = ev {
            match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let ElementState::Pressed = input.state {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Plus) => {
                                isolines.increase_precision(&background.grid, &function);
                            }
                            Some(VirtualKeyCode::Minus) => {
                                isolines.decrease_precision(&background.grid, &function);
                            }
                            Some(VirtualKeyCode::Up) => {
                                background.grid.dimensions.w += 5;
                                background.grid.dimensions.h += 5;
                                // function = PerlinNoise::new(background::GRID.dimensions);
                            }
                            Some(VirtualKeyCode::Down) => {
                                background.grid.dimensions.w -= 5;
                                background.grid.dimensions.h -= 5;
                                // function = PerlinNoise::new(background::GRID.dimensions);
                            }
                            Some(VirtualKeyCode::Space) => {
                                paused ^= true;
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
    });
}
