use std::ops::{Deref, DerefMut};
use std::time::Duration;
use crate::transformations::Transformation;
use crate::vertex::{Point, UVPoint};
use crate::Drawable;
use glium;
use glium::backend::Facade;
use glium::texture::MipmapsOption;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{DrawParameters, Frame, Program, Surface};
use image;
use nalgebra::Matrix3;

pub struct Rect {
    pub tl: (f32, f32),
    pub br: (f32, f32),
}

impl Rect {
    pub fn collides(&self, other: &Rect) -> bool {
        if self.tl.0 >= other.br.0 || other.tl.0 >= self.br.0 {
            return false
        }
        if self.br.1 >= other.tl.1 || other.br.1 >= self.tl.1 {
            return false
        }

        true
    }

    pub fn height(&self) -> f32 {
        self.br.1 - self.tl.1
    }

    pub fn width(&self) -> f32 {
        self.br.0 - self.tl.0
    }

    pub fn from_center(center: (f32, f32), width: f32, height: f32) -> Rect {
        Rect {
            tl: (center.0 - width / 2., center.1 + height / 2.),
            br: (center.1 - height / 2., center.1 - height / 2.),
        }
    }
}

pub struct PositionConsumer {
    positions: Vec<f32>
}

impl PositionConsumer {
    pub fn new() -> PositionConsumer {
        PositionConsumer {
            positions: Vec::with_capacity(8)
        }
    }

    pub fn feed(&mut self, number: f32) -> bool {
        self.positions.push(number);

        let mut iter = self.positions.iter().rev();
        let values = [iter.next(), iter.next(), iter.next()];

        if values.iter().any(|value| value.is_none()) {
            return false;
        }

        let values = values.map(|value| value.unwrap());
        let last_dx = values[1] - values[0];
        let recent_dx = values[2] - values[1];

        if last_dx < 0f32 && recent_dx > 0f32 {
            return true
        }

        return false
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::Rect;

    #[test]
    fn collides() {
        let first = Rect {
            tl: (-1., 1.),
            br: (0., 0.),
        };

        let second = Rect {
            tl: (-0.5, 0.5),
            br: (0.5, -0.5),
        };

        assert_eq!(first.collides(&second), true);
    }

    #[test]
    fn doesnt_collide() {
        let first = Rect {
            tl: (-1., 1.),
            br: (0., 0.),
        };

        let second = Rect {
            tl: (0.5, 0.5),
            br: (0.5, -0.5),
        };

        assert_eq!(first.collides(&second), false);
    }
}


#[cfg(test)]
mod position_consumer_test {
    use crate::utils::PositionConsumer;

    #[test]
    fn feed_works() {
        let mut pc = PositionConsumer::new();

        assert_eq!(pc.feed(2f32), false);
        assert_eq!(pc.feed(1f32), false);
        assert_eq!(pc.feed(1f32), false);

    }
}