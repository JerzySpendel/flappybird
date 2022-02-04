use crate::{Drawable, Texture};
use glium::backend::Facade;
use glium::{Frame, Program};
use std::cell::RefCell;
use std::time::Duration;

#[derive(PartialEq)]
pub enum GameState {
    Rolling,
    Message,
    Over,
}
