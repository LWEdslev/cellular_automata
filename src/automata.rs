use graphics::rectangle;
use graphics::types::Rectangle;

pub struct Automata {
    grid: Vec<Vec<Cell>>,
}

impl Automata {
    pub fn new(size: usize) -> Automata {
        assert!(size > 0);
        Automata {
            grid: vec![vec![Cell::new(0.0, 0.0, 0.9); size]; size]
        }
    }

    pub fn update(&mut self) {
        let old_grid = self.grid.clone();

        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                //temporary garbage
                self.grid[y][x].change_color(0.5 + 3.0*(x as f32 / 100.0), 0.7 - 3.0*(y as f32 / 100.0), 0.2 - ((x as i32 - y as i32)as f32 / 100.0));
            }
        }
    }

    pub fn get_rectangle_grid(&self, x_pos: f64, y_pos: f64, width: f64, height: f64) -> Vec<Vec<Rectangle>> {
        let width = width / self.grid[0].len() as f64;
        let height = height / self.grid.len() as f64;

        let mut out = Vec::new();

        for y in 0..self.grid.len() {
            let mut row = Vec::new();
            for x in 0..self.grid[0].len() {
                let cell = &self.grid[y][x];
                let (x, y, width, height) = (x as f64, y as f64, width as f64, height as f64);
                row.push(cell.to_rectangle(x_pos + x * width, y_pos + y * height, width, height));
            }
            out.push(row);
        }
        out
    }

    pub fn cell_at(&self, x: usize, y: usize) -> &Cell {
        &self.grid[y][x]
    }
}

#[derive(Clone)]
pub struct Cell {
    r: f32,
    g: f32,
    b: f32,
}

impl Cell {
    fn new(r: f32, g: f32, b: f32) -> Cell {
        Cell { r, g, b }
    }

    pub fn color(&self) -> [f32; 4] {
        [self.r, self.g, self.b, 1.0]
    }

    pub fn to_rectangle(&self, x: f64, y: f64, width: f64, height: f64) -> Rectangle {
        rectangle::rectangle_by_corners(x, y, x + width, y + height)
    }

    fn change_color(&mut self, r: f32, g: f32, b: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_color() {
        assert_eq!([0.5, 0.4, 0.3, 1.0], Cell::new(0.5, 0.4, 0.3).color())
    }
}

