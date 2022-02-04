use std::cell::RefCell;
use std::time::Duration;
use glium::backend::Facade;
use glium::{Frame, Program};
use crate::{Drawable, Texture};

#[derive(PartialEq)]
pub enum GameState {
    Rolling,
    Message,
    Over,
}
