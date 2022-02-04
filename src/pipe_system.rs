use crate::bird::Bird;
use crate::score::Score;
use crate::texture::Texture;
use crate::utils::{PositionConsumer, Rect};
use crate::{Drawable, GameState};
use glium::backend::Facade;
use glium::{Frame, Program};
use nalgebra::min;
use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

static INIT_POSITION: f32 = 1.5;

pub struct Pipe {
    texture: Rc<RefCell<Texture>>,
    gap: f32,
    gap_y: f32,
    gap_x: f32,
}

impl Pipe {
    pub fn new(texture: &Rc<RefCell<Texture>>, gap: f32, gap_y: f32, x: f32) -> Pipe {
        Pipe {
            texture: texture.clone(),
            gap,
            gap_y,
            gap_x: x,
        }
    }

    pub fn get_rects(&self) -> [Rect; 2] {
        let mut texture = self.texture.borrow_mut();
        let texture_height = texture.get_height();
        let texture_width = texture.get_width();
        [
            Rect {
                tl: (self.gap_x, self.gap_y + self.gap / 2f32 + texture_height),
                br: (self.gap_x + texture_width, self.gap_y + self.gap / 2f32),
            },
            Rect {
                tl: (self.gap_x, self.gap_y - self.gap / 2f32),
                br: (
                    self.gap_x + texture_width,
                    self.gap_y - self.gap / 2f32 - texture_height,
                ),
            },
        ]
    }
}

impl Drawable for Pipe {
    fn draw(
        &self,
        mut frame: Frame,
        facade: &dyn Facade,
        program: &Program,
        state: &GameState,
    ) -> Frame {
        let mut texture = self.texture.borrow_mut();
        let texture_height = texture.get_height();
        texture.rotation = Some(std::f32::consts::PI);
        texture.set_pos((self.gap_x, self.gap_y + self.gap / 2f32 + texture_height));
        frame = texture.draw(frame, facade, program, state);

        texture.rotation = Some(0.);
        texture.set_pos((self.gap_x, self.gap_y - self.gap / 2f32));
        frame = texture.draw(frame, facade, program, state);
        frame
    }

    fn update(&mut self, dt: std::time::Duration, state: &mut GameState) {
        todo!()
    }
}

pub struct PipeSystem {
    texture: Rc<RefCell<Texture>>,
    pipes: Vec<Pipe>,
    distance: f32,
    score_system: Score,
    pc: PositionConsumer,
}

impl PipeSystem {
    pub fn new(display: &dyn Facade) -> PipeSystem {
        PipeSystem {
            texture: Rc::new(RefCell::new(Texture::new(
                "./assets/sprites/pipe-green.png",
                display,
                (0f32, 0f32),
                Some(0.20),
            ))),
            pipes: vec![],
            distance: 1f32,
            score_system: Score::new(0.1, display),
            pc: PositionConsumer::new(),
        }
    }

    pub fn init(&mut self) {
        let mut pipes = &mut self.pipes;
        pipes.push(Pipe::new(&self.texture, 0.5, 0., 1f32));
    }

    fn last_pipe(&self) -> Option<&Pipe> {
        let mut r_pipe: Option<&Pipe> = None;

        for pipe in &self.pipes {
            match r_pipe {
                None => {
                    r_pipe = Some(pipe);
                }
                Some(inner_pipe) => {
                    if inner_pipe.gap_x < pipe.gap_x {
                        r_pipe = Some(pipe);
                    }
                }
            }
        }

        r_pipe
    }

    fn add_pipe(&mut self) {
        let last_x: f32;

        match self.last_pipe() {
            Some(pipe) => {
                last_x = pipe.gap_x;
            }
            None => {
                last_x = 0f32;
            }
        }

        self.pipes.push(Pipe::new(
            &self.texture,
            0.5,
            rand::thread_rng().gen::<f32>() / 4.,
            INIT_POSITION,
        ));
    }

    fn should_add_pipe(&self) -> bool {
        match self.last_pipe() {
            Some(pipe) => (pipe.gap_x - INIT_POSITION).abs() > self.distance,
            None => true,
        }
    }

    pub fn check_points(&mut self, bird: &Bird) {
        let bird_rect = bird.get_rect();
        let middle_x = (bird_rect.tl.0 + bird_rect.br.0) / 2.;
        let min_dist = self
            .pipes
            .iter()
            .map(|pipe| (pipe.gap_x - middle_x).abs())
            .min_by(|x, y| x.partial_cmp(y).unwrap());

        match min_dist {
            Some(dist) => {
                if self.pc.feed(dist) {
                    self.score_system.increment();
                }
            }
            None => (),
        }
    }

    pub fn check_collision(&self, bird: &Rect) -> bool {
        for pipe in &self.pipes {
            for rect in pipe.get_rects() {
                if rect.collides(bird) {
                    return true;
                }
            }
        }
        false
    }
}

impl Drawable for PipeSystem {
    fn draw(
        &self,
        mut frame: Frame,
        facade: &dyn Facade,
        program: &Program,
        state: &GameState,
    ) -> Frame {
        for pipe in &self.pipes {
            frame = pipe.draw(frame, facade, program, state);
        }
        frame = self.score_system.draw(frame, facade, program, state);

        frame
    }

    fn update(&mut self, dt: std::time::Duration, state: &mut GameState) {
        if GameState::Rolling != *state {
            return;
        }

        for pipe in &mut self.pipes {
            pipe.gap_x -= dt.as_secs_f32() / 2.;
        }

        let mut pipes = std::mem::take(&mut self.pipes);
        let new_pipes: Vec<Pipe> = pipes.into_iter().filter(|pipe| pipe.gap_x > -2.).collect();

        self.pipes = new_pipes;

        if self.should_add_pipe() {
            self.add_pipe();
        }
    }
}
