extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;
use crate::utility::Drawable;

struct App<T: Drawable> {
    gl: GlGraphics, // OpenGL drawing backend.
    automata: T,
}

impl<T: Drawable> App<T> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            let c = c.trans(0.0, 0.0);

            let automata = &mut self.automata;

            let updates = automata.get_new_graphics(args.window_size[0], args.window_size[1]);

            for (rect, color) in updates {
                rectangle(color, rect, c.transform, gl);
            }
        });
        self.automata.update();
    }
}

pub fn run<T: Drawable>(automata: T, window_size: u32, fps: u64) {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Cellular Automata", [window_size, window_size])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        automata,
    };

    let mut events = Events::new(EventSettings {
        max_fps: fps,
        ups: fps,
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
