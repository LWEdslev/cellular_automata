use graphics::types::{Rectangle, Color};

pub trait Drawable {
    fn update(&mut self);
    fn get_new_graphics(&self, width: f64, height: f64) -> Vec<(Rectangle, Color)>;
}
