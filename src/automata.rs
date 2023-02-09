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
        self.updated_cells.clear();

        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {

                let cell = &self.grid[y][x];

                let (old_b, old_active) = (cell.old_b, cell.old_active);

                let mut cell = &mut self.grid[y][x];

                cell.old_b = cell.b;
                cell.old_active = cell.active;

                let cell = &self.grid[y][x];

                let neighbours = self.get_neighbours(x, y);

                let neighbour_birth_sum = neighbours
                    .iter()
                    .filter(|cell| cell.old_active)
                    .count();



                let (alive, changed) = match neighbour_birth_sum {
                    2 => (cell.active, false),
                    3 => (true, !cell.active),
                    _ => (false, cell.active),
                };

                let cooldown = (!alive && changed) || old_b > 0.;

                let (red, green, blue, updated) = if alive { //is alive and red
                    (1., 0., 0., old_active)
                } else if cooldown { //in blue cooldown period
                    let just_died = !alive && changed;
                    let blue = if just_died { 1. } else { old_b - 0.1 };
                    let (red, green) = (0., 0.);
                    (red, green, blue, true)
                } else { //dead and nothing changed
                    (0., 0., 0., false)
                };

                if updated { //if the cell has been updated it should be added to the updated cells
                    self.updated_cells.push(Point(x, y));
                }

                //updating attributes of cell
                let mut cell = &mut self.grid[y][x];

                cell.active = alive;
                cell.change_color(red, green, blue);
            }
        }
    }

    fn get_new_graphics(&self, width: f64, height: f64) -> Vec<(Rectangle, Color)> {
        let width = width / self.grid[0].len() as f64;
        let height = height / self.grid.len() as f64;

        let mut out = Vec::new();

        for Point(x,y) in self.updated_cells.iter() {
            let cell = &self.grid[*y][*x];
            let (x, y, width, height) = ((*x as f64)*width, (*y as f64)*height, width, height);

            out.push((rectangle::rectangle_by_corners(x, y, x + width, y + height), cell.color()));
        }

        out
    }
}

impl Automata {
    pub fn new(size: usize) -> Automata {
        assert!(size > 0);

        let grid = vec![vec![Cell::new(0., 0., 0.); size]; size];
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
        cell.change_color(1., 0., 0.);
        cell.active = true;
    }
}

#[derive(Clone)]
pub struct Cell {
    r: f32,
    g: f32,
    b: f32,
    pub active: bool,
    old_b: f32,
    old_active: bool,
}

impl Cell {
    fn new(r: f32, g: f32, b: f32) -> Cell {
        Cell {
            r,
            g,
            b,
            active: false,
            old_b: 0.,
            old_active: false,
        }
    }

    pub fn color(&self) -> Color {
        [self.r, self.g, self.b, 1.]
    }

    fn change_color(&mut self, r: f32, g: f32, b: f32) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}
