use glium::backend::Facade;
use glium::{Frame, Program};
use crate::GameState;
use crate::utils::Rect;

pub trait Drawable {
    fn draw(
        &self,
        frame: Frame,
        facade: &dyn Facade,
        program: &Program,
        state: &GameState,
    ) -> Frame;

    fn update(&mut self, dt: std::time::Duration);
}