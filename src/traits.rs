use glium::backend::Facade;
use glium::{Frame, Program};
use crate::utils::Rect;

pub trait Drawable<'b, 'c> {
    fn draw(
        &self,
        frame: Frame,
        facade: &'b dyn Facade,
        program: &'c Program,
    ) -> Frame;

    fn update(&mut self, dt: std::time::Duration);
}