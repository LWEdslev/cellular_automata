use graphics::types::{Rectangle, Color};

pub const DEFAULT_FPS: u64 = 60;
pub const DEFAULT_WINDOW_SIZE: u64 = 500;
pub const DEFAULT_GRID_SIZE: u64 = 100;


pub trait Drawable {
    fn update(&mut self);
    fn get_new_graphics(&mut self, width: f64, height: f64) -> Vec<(Rectangle, Color)>;
}
