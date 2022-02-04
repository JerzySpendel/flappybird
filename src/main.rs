mod background;
mod bird;
mod message;
mod over;
mod pipe_system;
mod score;
mod state;
mod texture;
mod traits;
mod transformations;
mod uniforms;
mod utils;
mod vertex;

use crate::message::Message;
use crate::pipe_system::PipeSystem;
use crate::state::GameState;
use crate::texture::Texture;
use crate::traits::Drawable;
use crate::utils::Rect;
use glium;
use glium::Surface;
use glium::{glutin, Display, Frame, Program};
use over::GameOverLayer;
use std::time::Duration;

fn main() {
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(500, 500))
        .with_title("Hello world");
    let cb = glium::glutin::ContextBuilder::new();
    let mut display = glium::Display::new(wb, cb, &events_loop).unwrap();
    let program = glium::Program::from_source(
        &display,
        &std::fs::read_to_string("./vert.shader").unwrap(),
        &std::fs::read_to_string("./frag.shader").unwrap(),
        None,
    )
    .unwrap();
    let mut state = GameState::Message;
    let mut game_over_layer = GameOverLayer::new(&display);
    let mut message_layer = message::Message::new(&display);

    let mut background =
        background::Background::new("./assets/sprites/background-day.png", 1.4, &display, 0.05);
    let mut base = background::Background::new("./assets/sprites/base.png", 1.4, &display, 0.2);
    let mut bird = bird::Bird::new(&display);
    let mut pipe_system = PipeSystem::new(&display);
    pipe_system.init();

    let mut last_time = std::time::Instant::now();

    events_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => (),
            },
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::Key(input) => match input.virtual_keycode {
                    Some(keycode) => match keycode {
                        glutin::event::VirtualKeyCode::Space => {
                            bird.space_hit();
                        }
                        _ => {}
                    },
                    None => {}
                },
                _ => {}
            },
            _ => (),
        }

        let now = std::time::Instant::now();
        let mut dt = (now - last_time);
        last_time = now;

        let mut layers: &mut [&mut dyn Drawable] = &mut [
            &mut background,
            &mut pipe_system,
            &mut base,
            &mut bird,
            &mut game_over_layer,
            &mut message_layer,
        ];

        let mut frame = display.draw();
        frame.clear_color(1., 0., 0., 1.);

        for layer in layers.iter_mut() {
            frame = layer.draw(frame, &display, &program, &state);
            layer.update(dt, &mut state);
        }

        frame.finish().unwrap();

        let hittable: &[&dyn Fn(&Rect) -> bool] = &[
            &|r: &Rect| { r.tl.1 >= 1. || r.br.1 <= -1. },
            &|r: &Rect| { pipe_system.check_collision(r)},
        ];

        if hittable.iter().any(|hittable| hittable(&bird.get_rect())) {
            state = GameState::Over;
        }

        pipe_system.check_points(&bird);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(20),
        );
    });
}
