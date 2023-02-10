use graphics::rectangle;
use graphics::types::{Color, Rectangle};
use crate::utility::Drawable;

#[derive(Clone)]
struct Point(usize, usize);

#[derive(Clone)]
pub struct Automata {
    grid: Vec<Vec<Cell>>,
    updated_cells: Vec<Point>,
}

impl Drawable for Automata {
    fn update(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                let neighbour_birth_sum = self.get_neighbours(x, y)
                    .iter()
                    .filter(|cell| cell.old_active)
                    .count();

                let cell = &mut self.grid[y][x];

                let (alive, changed) = match neighbour_birth_sum {
                    2 => (cell.active, false),
                    3 => (true, !cell.active),
                    _ => (false, cell.active),
                };

                if changed || cell.cooldown > 0 { //if the cell has been updated it should be added to the updated cells
                    self.updated_cells.push(Point(x, y));
                    cell.active = alive;
                    cell.update_cooldown();
                }
            }
        }

        for Point(x, y) in &self.updated_cells {
            let cell = &mut self.grid[*y][*x];
            cell.old_active = cell.active;
        }
    }

    fn get_new_graphics(&mut self, width: f64, height: f64) -> Vec<(Rectangle, Color)> {
        let width = width / self.grid[0].len() as f64;
        let height = height / self.grid.len() as f64;

        let mut out = Vec::new();

        for Point(x,y) in self.updated_cells.iter() {
            let cell = &self.grid[*y][*x];
            let (x, y, width, height) = ((*x as f64)*width, (*y as f64)*height, width, height);

            out.push((rectangle::rectangle_by_corners(x, y, x + width, y + height), cell.color()));
        }

        self.updated_cells.clear();

        out
    }
}

impl Automata {
    pub fn new(size: usize) -> Automata {
        assert!(size > 0);

        let grid = vec![vec![Cell::new(false); size]; size];
        let updated_cells = vec![];

        Automata {
            grid,
            updated_cells,
        }
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<&Cell> {
        let points = [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
        let grid = &self.grid;
        let (x, y) = (x as isize, y as isize);
        let (x_range, y_range)= (0..grid[0].len() as isize, 0..grid.len() as isize);

        points.iter()
            .map(|(x_off, y_off)| (x + x_off, y + y_off))
            .filter(|(x, y)| x_range.contains(x) && y_range.contains(y))
            .map(|(x, y)| &grid[y as usize][x as usize])
            .collect()
    }

    pub fn birth_cell_at(&mut self, x: usize, y: usize) {
        let cell = &mut self.grid[y][x];
        cell.active = true;
        cell.old_active = true;
        self.updated_cells.push(Point(x, y));
    }
}

#[derive(Clone)]
pub struct Cell {
    active: bool,
    old_active: bool,
    cooldown: u8,
}

impl Cell {
    fn new(active: bool) -> Cell {
        Cell {
            active,
            old_active: active,
            cooldown: 0,
        }
    }

    fn update_cooldown(&mut self) {
        self.cooldown = if !self.active && self.old_active {
            10
        } else if self.cooldown > 0 {
            self.cooldown - 1
        } else {
            0
        }

    }

    pub fn color(&self) -> Color {
        if self.active {
            [1., 0., 0., 1.]
        } else if self.cooldown > 0 {
            [0., 0., self.cooldown as f32 / 10., 1.]
        } else {
            [0., 0., 0., 1.]
        }
    }
}
