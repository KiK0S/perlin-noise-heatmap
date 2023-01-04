use crate::draw::*;
use crate::function::PerlinNoise;
use crate::grid::{Dimensions, Grid};
use crate::isoline::Isolines;
use glium::Display;
use glium::Surface;

pub const BACKGROUND: Grid = Grid {
    x0: -1.0,
    x1: 1.0,
    y0: -1.0,
    y1: 1.0,
    dimensions: Dimensions { w: 100, h: 100 },
};

pub const GRID: Grid = Grid {
    x0: -1.0,
    x1: 1.0,
    y0: -1.0,
    y1: 1.0,
    dimensions: Dimensions { w: 4, h: 4 },
};

pub fn draw_background(
    display: &mut Display,
    _time: f32,
    function: &PerlinNoise,
    isolines: &mut Isolines,
) {
    // let red = (time as f32).sin() / 2.0 + 1.0;
    // let blue = (time as f32 + PI / 3.0).sin() / 2.0 + 1.0;
    // let green = (time as f32 + PI * 2.0 / 3.0).sin() / 2.0 + 1.0;
    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);
    for (x, y) in BACKGROUND.iterator() {
        let value = function.get_value(x, y, &BACKGROUND);
        draw_square(
            display,
            &mut target,
            x,
            y,
            BACKGROUND.get_cell_width() as f32,
            (value + 0.4, 0.2, 0.05),
        );
    }

    isolines.draw(display, &mut target, &BACKGROUND, function);
    // for x in GRID.verticals() {
    //     draw_vertical(display, &mut target, x);
    // }
    // for y in GRID.verticals() {
    //     draw_horizontal(display, &mut target, y);
    // }
    // function.debug_draw(display, &mut target, &BACKGROUND);
    target.finish().unwrap();
}
