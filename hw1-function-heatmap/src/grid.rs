#[derive(Clone, Copy)]
pub struct Dimensions {
    pub w: i32,
    pub h: i32,
}

#[derive(Clone, Copy)]
pub struct Grid {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub dimensions: Dimensions,
}

impl Grid {
    pub fn new(mut x0: f32, mut x1: f32, mut y0: f32, mut y1: f32, dimensions: Dimensions, window: Dimensions) -> Self {
        let ratio = (y1 - y0) / (x1 - x0);
        let window_ratio = (window.h as f32) / (window.w as f32);

        if ratio > window_ratio {
            // image is too high
            x0 = -window_ratio/ratio;
            x1 = window_ratio/ratio;
        }
        if ratio < window_ratio {
            // image is too wide
            y0 = -1.0/window_ratio*ratio;
            y1 = 1.0/window_ratio*ratio;
        }   
        Self {
            x0,
            x1,
            y0,
            y1,
            dimensions,
        }
    }

    pub fn get_cell_width(&self) -> f32 {
        (self.x1 - self.x0) / self.dimensions.w as f32
    }

    pub fn get_cell_height(&self) -> f32 {
        (self.y1 - self.y0) / self.dimensions.h as f32
    }

    pub fn verticals(&self) -> Vec<f32> {
        (0..self.dimensions.w + 1)
            .map(|iter| self.x0 + iter as f32 * self.get_cell_width())
            .collect()
    }

    pub fn horizontals(&self) -> Vec<f32> {
        (0..self.dimensions.h + 1)
            .map(|iter| self.y0 + iter as f32 * self.get_cell_height())
            .collect()
    }

    pub fn get_point(&self, x: i32, y: i32) -> (f32, f32) {
        (
            self.x0 + x as f32 * self.get_cell_width(),
            self.y0 + y as f32 * self.get_cell_height(),
        )
    }

    pub fn get_point_rev(&self, x: f32, y: f32) -> (i32, i32) {
        (
            ((x - self.x0 + 0.001) / self.get_cell_width()) as i32,
            ((y - self.y0 + 0.001) / self.get_cell_height()) as i32,
        )
    }
}

impl<'a> Grid {
    pub fn iterator(&'a self, extra_bound: bool) -> GridIterator {
        GridIterator {
            idx: 0,
            grid: self,
            extra_bound,
        }
    }
}

pub struct GridIterator<'a> {
    idx: i32,
    grid: &'a Grid,
    extra_bound: bool,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (f32, f32);
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.idx;
        if !self.extra_bound {
            if t == self.grid.dimensions.w * self.grid.dimensions.h {
                return None;
            }
            let x = t / self.grid.dimensions.h;
            let y = t % self.grid.dimensions.h;
            self.idx += 1;
            Some(self.grid.get_point(x, y))
        } else {
            if t == (self.grid.dimensions.w + 1) * (self.grid.dimensions.h + 1) {
                return None;
            }
            let x = t / (self.grid.dimensions.h + 1);
            let y = t % (self.grid.dimensions.h + 1);
            self.idx += 1;
            Some(self.grid.get_point(x, y))
        }
    }
}
