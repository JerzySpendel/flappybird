use std::cell::{RefCell, RefMut};
use std::time::Duration;
use crate::traits::Drawable;
use glium::backend::Facade;
use glium::{uniform, BlendingFunction, DrawParameters, Frame, Program, Surface};
use nalgebra::{Matrix, Matrix2, OMatrix, Rotation2};
use crate::texture::Texture;
use crate::pipe_system::Pipe;
use crate::utils::Rect;

pub struct Bird {
    textures: [RefCell<Texture>; 3],
    start_time: std::time::Instant,
    last_update: Option<std::time::Instant>,
    y_speed: f32,
    y_position: f32,
}

impl Bird {
    pub fn new(display: &dyn glium::backend::Facade) -> Bird {
        Bird {
            textures: [
                RefCell::new(Texture::new("./assets/sprites/bluebird-downflap.png", display, (0., 0.), Some(0.2))),
                RefCell::new(Texture::new("./assets/sprites/bluebird-midflap.png", display, (0., 0.), Some(0.2))),
                RefCell::new(Texture::new("./assets/sprites/bluebird-upflap.png", display, (0., 0.), Some(0.2))),
            ],
            start_time: std::time::Instant::now(),
            last_update: None,
            y_speed: 0.,
            y_position: 0.,
        }
    }

    fn current_texture(&self) -> RefMut<Texture> {
        let elapsed = (std::time::Instant::now() - self.start_time).as_secs_f32() % 1.0;
        match elapsed {
            x if x >= 0. && x < 0.33 => self.textures[0].borrow_mut(),
            x if x >= 0.33 && x < 0.66 => self.textures[1].borrow_mut(),
            x if x >= 0.66 && x <= 1. => self.textures[2].borrow_mut(),
            _ => self.textures[0].borrow_mut(),
        }
    }

    pub fn space_hit(&mut self) {
        self.y_speed = 1.;
    }

    fn get_rotation(&self) -> f32 {
        let S = |x: f32| -> f32 { -1. + 2.5 / (1. + (-x).exp()) };
        S(self.y_speed).clamp(-2., 2.)
    }

    pub fn get_rect(&self) -> Rect {
        self.current_texture().get_rect()
    }

}

impl Drawable for Bird {
    fn draw(
        &self,
        mut frame: Frame,
        facade: &dyn Facade,
        program: &Program,
    ) -> Frame {
        let mut texture = self.current_texture();
        texture.set_rotation(self.get_rotation());
        texture.set_pos((0., self.y_position));
        frame = texture.draw(frame, facade, program);
        frame
    }

    fn update(&mut self, dt: Duration) {
        match self.last_update {
            Some(update_time) => {
                let now = std::time::Instant::now();
                self.y_speed -= dt.as_secs_f32() * 2.;
                self.y_position += self.y_speed * dt.as_secs_f32();
                self.y_position = self.y_position.clamp(-1., 1.);

                self.last_update = Some(now);
            }
            None => {
                self.last_update = Some(std::time::Instant::now());
            }
        }

    }
}
