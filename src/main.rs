mod automata;
mod gui;
mod utility;

use rand::Rng;
use crate::automata::Automata;
use std::env;
use crate::utility::{DEFAULT_FPS, DEFAULT_GRID_SIZE, DEFAULT_WINDOW_SIZE};

fn main() {

    let args: Vec<String> = env::args().collect();

    let get_value = |search_param: &str, default: u64| {
        match args.iter().find(|s| s.to_lowercase().contains(&format!("{}=", search_param))) {
            Some(s) => s.split('=').last().unwrap().parse().unwrap_or_else(|_| panic!("There must be a number after {}=", search_param)),
            None => default,
    }};

    let fps = get_value("fps", DEFAULT_FPS);

    let grid_size = get_value("grid", DEFAULT_GRID_SIZE) as usize;

    let window_size = get_value("window", DEFAULT_WINDOW_SIZE) as u32;


    let mut automata = Automata::new(grid_size);

    let mut rng = rand::thread_rng();

    for x in 0..grid_size {
        for y in 0..grid_size {
            if rng.gen_bool(0.3) {
                automata.birth_cell_at(x, y);
            }
        }
    }

    gui::run(automata, window_size, fps);
}
