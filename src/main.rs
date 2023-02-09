mod automata;
mod gui;
mod utility;

use rand::Rng;
use crate::automata::Automata;

fn main() {

    let size = 200;

    let mut automata = Automata::new(size);

    let mut rng = rand::thread_rng();

    for x in 0..size {
        for y in 0..size {
            if rng.gen::<f64>() < 0.3 {
                automata.birth_cell_at(x, y);
            }
        }
    }

    gui::run(automata, 800, 60);
}