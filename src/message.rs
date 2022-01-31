use std::cell::RefCell;
use std::time::Duration;
use glium::backend::Facade;
use glium::{Frame, Program};
use crate::{Drawable, GameState, Texture};

pub struct Message {
    texture: RefCell<Texture>,
}

impl Message {
    pub fn new(display: &dyn Facade) -> Self {
        Message {
            texture: RefCell::new(Texture::new("./assets/sprites/message.png", display, (0., 0.), None))
        }
    }
}

impl Drawable for Message {
    fn draw(&self, frame: Frame, facade: &dyn Facade, program: &Program, state: &GameState) -> Frame {
        self.texture.borrow().draw(frame, facade, program, state)
    }

    fn update(&mut self, dt: Duration) {
    }
}