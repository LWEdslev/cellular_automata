use graphics::rectangle;
use graphics::types::Rectangle;

#[derive(Clone)]
struct Point(usize, usize);

#[derive(Clone)]
pub struct Automata {
    grid: Vec<Vec<Cell>>,
    updated_cells: Vec<Point>,
}

impl Automata {
    pub fn new(size: usize) -> Automata {
        assert!(size > 0);
        Automata {
            grid: vec![vec![Cell::new(0.0, 0.0, 0.0); size]; size],
            updated_cells: vec![],
        }
    }

    pub fn update(&mut self) {
        let old = self.clone();

        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {

                let old_cell = &old.grid[y][x];

                let neighbours = old.get_neighbours(x, y);

                let neighbour_birth_sum = neighbours
                    .iter()
                    .filter(|cell| cell.active)
                    .count();

                let cell = &self.grid[y][x];

                let (alive, changed) = match neighbour_birth_sum {
                    2 => (cell.active, false),
                    3 => (true, !cell.active),
                    _ => (false, cell.active),
                };

                let cooldown = (!alive && changed) || old_cell.b > 0.0;

                let (red, green, blue, updated) = if alive {
                    let red = 1.0;
                    let blue = 0.0;
                    let green = 0.0;
                    (red, green, blue, old_cell.active)
                } else if cooldown {
                    let just_died = !alive && changed;
                    let blue = if just_died { 1.0 } else { old_cell.b - 0.1 };
                    let (red, green) = (0.0, 0.0);
                    (red, green, blue, true)
                } else {
                    (0.0, 0.0, 0.0, false)
                };

                if updated {
                    self.updated_cells.push(Point(x, y));
                }

                self.grid[y][x].active = alive;
                self.grid[y][x].change_color(red, green, blue);
            }
        }
    }

    pub fn get_neighbours(&self, x: usize, y: usize) -> Vec<&Cell> {
        let points = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

        let grid = &self.grid;

        let mut buf = Vec::with_capacity(points.len());
        for (x_offset, y_offset) in points {
            let (x, y) = (x as isize + x_offset ,y as isize + y_offset);
            if x >= 0 && y >= 0 && y < grid.len() as isize && x < grid[0].len() as isize {
                buf.push(&grid[y as usize][x as usize])
            }
        }

        buf
    }

    pub fn get_rectangle_grid(&self, x_pos: f64, y_pos: f64, width: f64, height: f64) -> Vec<(Rectangle, &Cell)> {
        let width = width / self.grid[0].len() as f64;
        let height = height / self.grid.len() as f64;

        let mut out = Vec::new();

        for Point(x,y) in &self.updated_cells {
            let cell = &self.grid[*y][*x];
            let (x, y, width, height) = (*x as f64, *y as f64, width as f64, height as f64);
            out.push((cell.to_rectangle(x_pos + x * width, y_pos + y * height, width, height), cell));
        }

        out
    }

    pub fn cell_at(&self, x: usize, y: usize) -> &Cell {
        &self.grid[y][x]
    }

    pub fn birth_cell_at(&mut self, x: usize, y: usize) {
        let cell = &mut self.grid[y][x];
        cell.change_color(1.0, 0.0, 0.0);
        cell.active = true;
    }
}

#[derive(Clone)]
pub struct Cell {
    r: f32,
    g: f32,
    b: f32,
    pub active: bool,
}

impl Cell {
    fn new(r: f32, g: f32, b: f32) -> Cell {
        Cell { r, g, b, active: false }
    }

    pub fn color(&self) -> [f32; 4] {
        [self.r, self.g, self.b, 1.0]
    }

    pub fn to_rectangle(&self, x: f64, y: f64, width: f64, height: f64) -> Rectangle {
        rectangle::rectangle_by_corners(x, y, x + width, y + height)
    }

    fn set_active(&mut self, b: bool) {
        self.active = b;
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

