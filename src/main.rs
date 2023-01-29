mod automata;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::automata::Automata;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    automata: Automata,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            let c = c.trans(0.0, 0.0);
            self.automata.update();
            let automata = &self.automata;

            let grid = automata.get_rectangle_grid(10.0, 10.0, 480.0, 480.0);

            for y in 0..grid.len() {
                for x in 0..grid[0].len() {
                    let cell = automata.cell_at(x, y);
                    let color = cell.color();
                    let rect = grid[y][x];
                    rectangle(color, rect, c.transform, gl);
                }
            }

            //let rect = rectangle::rectangle_by_corners(0f64, 0f64, 100f64, 80f64);

            //rectangle(RED, rect, c.transform, gl);
            // Draw a box rotating around the middle of the screen.
            //rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {

    }
}





fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Cellular Automata", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        automata: Automata::new(30),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}