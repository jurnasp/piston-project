extern crate piston_window;

use crate::collider::Collider;
use crate::collides::Collides;
use crate::player::Action::NoMove;
use crate::player::State::Active;
use crate::position::Position;
use crate::settings;
use crate::settings::player;
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
    pub(crate) color: [f32; 4],
}

impl Collides for Player {
    fn get_collider(&mut self) -> &mut Collider {
        &mut self.collider
    }
}

impl Position for Player {
    fn set_position(&mut self, position: Vector2) {
        self.get_collider().set_position(position)
    }

    fn get_position(&mut self) -> Vector2 {
        self.get_collider().get_position()
    }
}

impl Player {
    pub fn new(position: Vector2) -> Self {
        Player {
            collider: Collider::new(position, settings::player::SIZE),
            horizontal: Active(NoMove),
            vertical: Active(NoMove),
            color: settings::color::WHITE,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut target = Vector2::new(0.0, 0.0);
        let current_position = self.get_position();
        match &self.horizontal {
            State::Active(action) => match action {
                Action::Minus => target.x = current_position.x - player::SPEED * dt,
                Action::Plus => target.x = current_position.x + player::SPEED * dt,
                Action::NoMove => target.x = current_position.x,
            },
            _ => {}
        }
        match &self.vertical {
            State::Active(action) => match action {
                Action::Minus => target.y = current_position.y + player::SPEED * dt,
                Action::Plus => target.y = current_position.y - player::SPEED * dt,
                Action::NoMove => target.y = current_position.y,
            },
            _ => {}
        }
        if current_position != target {
            self.set_position(Vector2::move_towards(
                current_position,
                target,
                settings::player::SPEED * dt,
            ));
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
                let position = self.get_position();
                // let square = rectangle::square(0.0, 0.0, player::SIZE);
                //
                // let (x, y) = (self.collider.position.x, self.collider.position.y);
                // let transform = c.transform.trans(x, y);
                //
                // rectangle(color::WHITE, square, transform, g);
                let rect = [
                    position.x - player::SIZE,
                    position.y - player::SIZE,
                    player::SIZE * 2.0,
                    player::SIZE * 2.0,
                ];

                ellipse(self.color, rect, c.transform, g);
            }
            State::Dead => {}
        }
    }

    pub fn damage(&mut self) {
        self.color[3] = 0.25
    }

    pub fn normal(&mut self) {
        self.color[3] = 1.0
    }
}
