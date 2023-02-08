mod automata;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;
use crate::automata::Automata;
use rand::Rng;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    automata: Automata,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            let c = c.trans(0.0, 0.0);

            let automata = &self.automata;

            let updates = automata.get_rectangle_grid(0.0, 0.0, 900.0 , 900.0);

            for (rect, color) in updates {
                rectangle(color, rect, c.transform, gl);
            }

            self.automata.update();
        });
    }
}





fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Cellular Automata", [900, 900])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let size = 300;

    let mut automata = Automata::new(size);

    let mut rng = rand::thread_rng();

    for x in 0..size {
        for y in 0..size {
            if rng.gen::<f64>() < 0.2 {
                automata.birth_cell_at(x, y);
            }
        }
    }

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        automata,
    };

    let mut events = Events::new(EventSettings {
        max_fps: 60,
        ups: 60,
        ups_reset: 2,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    });
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
    }
}