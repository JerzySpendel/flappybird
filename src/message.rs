use std::cell::RefCell;
use std::time::Duration;
use glium::backend::Facade;
use glium::{Frame, Program};
use crate::{Drawable, GameState, Texture};

pub struct Message {
    texture: RefCell<Texture>,
    time_elapsed: std::time::Duration,
}

impl Message {
    pub fn new(display: &dyn Facade) -> Self {
        Message {
            texture: RefCell::new(Texture::new("./assets/sprites/message.png", display, (0., 0.), None)),
            time_elapsed: Duration::new(0, 0),
        }
    }
}

impl Drawable for Message {
    fn draw(&self, mut frame: Frame, facade: &dyn Facade, program: &Program, state: &GameState) -> Frame {
        match state {
            GameState::Message => {
                frame = self.texture.borrow().draw(frame, facade, program, state)
            }
            _ => {}
        }
        frame
    }

    fn update(&mut self, dt: std::time::Duration, state: &mut GameState) {
        self.time_elapsed += dt;
        if self.time_elapsed > std::time::Duration::from_secs(1) {
            match *state {
                GameState::Message => {
                    *state = GameState::Rolling
                }
                _ => {}
            }
        }
    }
}