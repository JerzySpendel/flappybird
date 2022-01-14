use std::cell::RefCell;
use std::time::Duration;
use glium::backend::Facade;
use glium::{Frame, Program};
use crate::{Drawable, Texture};

pub enum GameState {
    Rolling,
    Over,
}


pub struct GameOverLayer {
    state: GameState,
    texture: RefCell<Texture>
}

impl GameOverLayer {
    pub fn new(state: GameState, display: &dyn Facade) -> GameOverLayer {
        let texture = RefCell::new(Texture::new("./assets/sprites/gameover.png", display, (0., 0.), Some(1.)));
        texture.borrow_mut().set_pos_center((0., 0.));

        GameOverLayer {
            state,
            texture
        }

    }

    pub fn set_end(&mut self) {
        self.state = GameState::Over;
    }

    pub fn game_ended(&self) -> bool {
        if let GameState::Over = self.state { true } else { false }
    }
}

impl<'b, 'c> Drawable<'b, 'c> for GameOverLayer{
    fn draw(&self, mut frame: Frame, facade: &'b dyn Facade, program: &'c Program) -> Frame {
        if self.game_ended() {
            let texture = self.texture.borrow();
            frame = texture.draw(frame, facade, program);
        }

        frame
    }

    fn update(&mut self, dt: Duration) {
    }
}
