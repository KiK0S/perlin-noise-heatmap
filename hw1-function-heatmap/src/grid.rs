pub struct Dimensions {
    pub w: i32,
    pub h: i32,
}

pub struct Grid {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub dimensions: Dimensions,
}

impl Grid {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, w: i32, h: i32) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            dimensions: Dimensions { w, h },
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
            .map(|iter| self.x0 + iter as f32 * (self.x1 - self.x0) / self.dimensions.w as f32)
            .collect()
    }

    pub fn horizontals(&self) -> Vec<f32> {
        (0..self.dimensions.h + 1)
            .map(|iter| self.y0 + iter as f32 * (self.y1 - self.y0) / self.dimensions.h as f32)
            .collect()
    }

    pub fn get_point(&self, x: i32, y: i32) -> (f32, f32) {
        (
            self.x0 + x as f32 * (self.x1 - self.x0) / self.dimensions.w as f32,
            self.y0 + y as f32 * (self.y1 - self.y0) / self.dimensions.h as f32,
        )
    }
}

impl<'a> Grid {
    pub fn iterator(&'a self) -> GridIterator {
        GridIterator { idx: 0, grid: self }
    }
}

pub struct GridIterator<'a> {
    idx: i32,
    grid: &'a Grid,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (f32, f32);
    fn next(&mut self) -> Option<Self::Item> {
        let t = self.idx;
        if t == self.grid.dimensions.w * self.grid.dimensions.h {
            return None;
        }
        self.idx += 1;
        let x = t / self.grid.dimensions.h;
        let y = t % self.grid.dimensions.h;
        Some(self.grid.get_point(x, y))
    }
}
