use crate::{Drawable, GameState, Texture};
use glium::backend::Facade;
use glium::{Frame, Program};
use std::cell::RefCell;
use std::time::Duration;

pub struct GameOverLayer {
    texture: RefCell<Texture>,
}

impl GameOverLayer {
    pub fn new(display: &dyn Facade) -> GameOverLayer {
        let texture = RefCell::new(Texture::new(
            "./assets/sprites/gameover.png",
            display,
            (0., 0.),
            Some(1.),
        ));
        texture.borrow_mut().set_pos_center((0., 0.));

        GameOverLayer { texture }
    }
}

impl Drawable for GameOverLayer {
    fn draw(
        &self,
        mut frame: Frame,
        facade: &dyn Facade,
        program: &Program,
        state: &GameState,
    ) -> Frame {
        match *state {
            GameState::Over => {
                let texture = self.texture.borrow();
                frame = texture.draw(frame, facade, program, state);
            }
            _ => {}
        }

        frame
    }

    fn update(&mut self, dt: std::time::Duration, state: &mut GameState) {}
}
