use crate::utils::Rect;
use crate::GameState;
use glium::backend::Facade;
use glium::{Frame, Program};
use std::time::Duration;

pub trait Drawable {
    fn draw(
        &self,
        frame: Frame,
        facade: &dyn Facade,
        program: &Program,
        state: &GameState,
    ) -> Frame;

    fn update(&mut self, dt: std::time::Duration, state: &mut GameState);
}
