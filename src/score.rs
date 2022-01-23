use std::cell::RefCell;
use std::path::Path;
use std::time::Duration;
use arrayvec::ArrayVec;
use glium::backend::Facade;
use glium::{Frame, Program};
use crate::{Drawable, Texture};
use crate::utils::Rect;

pub struct Score {
    score: u16,
    numbers: [RefCell<Texture>; 10],
    number_rect: Rect,
}

impl Score {
    pub fn new(number_width: f32, display: &dyn Facade) -> Score {
        let mut path = Path::new("./assets/sprites");
        let mut textures_array: ArrayVec<RefCell<Texture>, 10> = ArrayVec::new();
        for i in 0..10 {
            textures_array.push(RefCell::new(Texture::new(
                path.join(i.to_string() + ".png").to_str().unwrap(),
                display, (0.0, 0.0), Some(number_width))
            ));
        }

        let numbers = textures_array.into_inner().unwrap();
        let number_rect = numbers.get(0usize).unwrap().borrow().get_rect();

        Score {
            score: 0,
            numbers,
            number_rect,
        }
    }

    pub fn increment(&mut self) {
        self.score += 1;
    }
}

impl Drawable for Score {
    fn draw(&self, frame: Frame, facade: &dyn Facade, program: &Program) -> Frame {
        let digits = self.score.to_string().chars().map(|char|{
            char.to_digit(10).unwrap().try_into().unwrap()
        }).collect::<Vec<u16>>();

        let mut frame = frame;
        for (index, digit) in digits.iter().rev().enumerate() {
            let mut texture = self.numbers.get(usize::try_from(*digit).unwrap()).unwrap().borrow_mut();
            texture.set_pos((1f32 - (index + 1) as f32 * self.number_rect.width(), 1f32));
            frame = texture.draw(frame, facade, program);
        }
        frame
    }

    fn update(&mut self, dt: Duration) {
        todo!()
    }
}