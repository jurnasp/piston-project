extern crate piston_window;

use crate::collider::Collider;
use crate::collides::Collides;
use crate::player::Action::NoMove;
use crate::player::State::Active;
use crate::settings;
use crate::settings::{color, player};
use crate::vector2::Vector2;
use opengl_graphics::GlGraphics;
use piston_window::{ellipse, Context};

#[derive(Clone, Copy)]
pub enum KeyState {
    Pressed,
    NotPressed,
}

enum Action {
    NoMove,
    Plus,
    Minus,
}

#[allow(dead_code)]
enum State {
    Active(Action),
    Dead,
}

pub struct Player {
    pub collider: Collider,
    horizontal: State,
    vertical: State,
}

impl Collides for Player {
    fn collides_with<C: Collides>(&self, other: &C) -> bool {
        self.collider.collides_with(other.get_collider())
    }

    fn get_collider(&self) -> &Collider {
        &self.collider
    }
}

impl Player {
    pub fn new(position: Vector2) -> Self {
        Player {
            collider: Collider::new(position, settings::player::SIZE),
            horizontal: Active(NoMove),
            vertical: Active(NoMove),
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut target = Vector2::new(0.0, 0.0);
        match &self.horizontal {
            State::Active(action) => match action {
                Action::Minus => target.x = self.collider.position.x - player::SPEED * dt,
                Action::Plus => target.x = self.collider.position.x + player::SPEED * dt,
                Action::NoMove => target.x = self.collider.position.x,
            },
            _ => {}
        }
        match &self.vertical {
            State::Active(action) => match action {
                Action::Minus => target.y = self.collider.position.y + player::SPEED * dt,
                Action::Plus => target.y = self.collider.position.y - player::SPEED * dt,
                Action::NoMove => target.y = self.collider.position.y,
            },
            _ => {}
        }
        if self.collider.position != target {
            self.collider.position =
                Vector2::move_towards(self.collider.position, target, settings::player::SPEED * dt);
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
                // let square = rectangle::square(0.0, 0.0, player::SIZE);
                //
                // let (x, y) = (self.collider.position.x, self.collider.position.y);
                // let transform = c.transform.trans(x, y);
                //
                // rectangle(color::WHITE, square, transform, g);
                let rect = [
                    self.collider.position.x - player::SIZE,
                    self.collider.position.y - player::SIZE,
                    player::SIZE * 2.0,
                    player::SIZE * 2.0,
                ];

                ellipse(color::WHITE, rect, c.transform, g);
            }
            State::Dead => {}
        }
    }
}
