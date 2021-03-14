extern crate piston_window;

use crate::settings;
use crate::settings::{color, player};
use crate::vector2::Vector2;
use opengl_graphics::GlGraphics;
use piston_window::*;

#[derive(Clone, Copy)]
pub enum KeyState {
    Pressed,
    NotPressed,
}

pub enum Action {
    NoMove,
    Plus,
    Minus,
}

#[allow(dead_code)]
pub enum State {
    Active(Action),
    Dead,
}

pub struct Player {
    pub position: Vector2,
    pub horizontal: State,
    pub vertical: State,
}

impl Player {
    pub fn update(&mut self, dt: f64) {
        let mut target = Vector2 {
            ..Default::default()
        };
        match &self.horizontal {
            State::Active(action) => match action {
                Action::Minus => target.x = self.position.x - player::SPEED * dt,
                Action::Plus => target.x = self.position.x + player::SPEED * dt,
                Action::NoMove => target.x = self.position.x,
            },
            _ => {}
        }
        match &self.vertical {
            State::Active(action) => match action {
                Action::Minus => target.y = self.position.y + player::SPEED * dt,
                Action::Plus => target.y = self.position.y - player::SPEED * dt,
                Action::NoMove => target.y = self.position.y,
            },
            _ => {}
        }
        if self.position != target {
            self.position =
                Vector2::move_towards(self.position, target, settings::player::SPEED * dt);
        }
    }

    pub fn input(&mut self, left: KeyState, right: KeyState, up: KeyState, down: KeyState) {
        match self.horizontal {
            State::Active(_) => match (left, right) {
                (KeyState::Pressed, KeyState::NotPressed) => {
                    self.horizontal = State::Active(Action::Minus)
                }
                (KeyState::NotPressed, KeyState::Pressed) => {
                    self.horizontal = State::Active(Action::Plus)
                }
                _ => self.horizontal = State::Active(Action::NoMove),
            },
            _ => {}
        }
        match self.vertical {
            State::Active(_) => match (down, up) {
                (KeyState::Pressed, KeyState::NotPressed) => {
                    self.vertical = State::Active(Action::Minus)
                }
                (KeyState::NotPressed, KeyState::Pressed) => {
                    self.vertical = State::Active(Action::Plus)
                }
                _ => self.vertical = State::Active(Action::NoMove),
            },
            _ => {}
        }
    }

    pub fn draw(&mut self, c: Context, g: &mut GlGraphics) {
        match &self.horizontal {
            State::Active(_action) => {
                let square = rectangle::square(0.0, 0.0, player::SIZE);
                let (x, y) = (self.position.x, self.position.y);
                let transform = c.transform.trans(x, y);

                rectangle(color::WHITE, square, transform, g);
            }
            State::Dead => {}
        }
    }
}
