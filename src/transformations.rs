use nalgebra::{Matrix, Matrix3, Vector2};

#[derive(Debug)]
pub struct Transformation {
    pub scale: Option<(f32, f32)>,
    pub rotation: Option<f32>,
    pub translation: Option<[f32; 2]>,
}

impl Transformation {
    pub fn new() -> Transformation {
        Transformation {
            scale: None,
            rotation: None,
            translation: None,
        }
    }

    pub fn get_matrix(&self) -> Matrix3<f32> {
        let mut matrix = Matrix3::<f32>::identity();
        match self.scale {
            Some(scale) => unsafe {
                *matrix.get_unchecked_mut(0) *= scale.0;
                *matrix.get_unchecked_mut(4) *= scale.1;
            },
            None => (),
        }

        match self.rotation {
            Some(rotation) => {
                matrix = Matrix3::<f32>::new_rotation(rotation) * matrix;
            }
            None => {}
        }

        match self.translation {
            Some(translation) => {
                matrix = Matrix3::<f32>::new_translation(&Vector2::<f32>::new(
                    translation[0],
                    translation[1],
                )) * matrix;
            }
            None => {}
        }

        matrix
    }
}
