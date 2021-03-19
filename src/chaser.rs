use crate::collider::Collider;
use crate::collides::Collides;
use crate::position::Position;
use crate::settings;
use crate::vector2::Vector2;
use opengl_graphics::GlGraphics;
use piston_window::{ellipse, Context};

pub struct Chaser {
    pub collider: Collider,
}

impl Collides for Chaser {
    fn get_collider(&mut self) -> &mut Collider {
        &mut self.collider
    }
}

impl Position for Chaser {
    fn set_position(&mut self, position: Vector2) {
        self.get_collider().set_position(position)
    }

    fn get_position(&mut self) -> Vector2 {
        self.get_collider().get_position()
    }
}

impl Chaser {
    pub(crate) fn new(position: Vector2) -> Self {
        Chaser {
            collider: Collider::new(position, settings::chaser::SIZE),
        }
    }

    pub fn update(&mut self, dt: f64, target_position: Vector2) {
        let position = self.get_position();
        if position == target_position {
            return;
        }

        self.set_position(Vector2::move_towards(
            position,
            target_position,
            settings::chaser::SPEED * dt,
        ));
    }

    pub fn draw(&mut self, c: Context, g: &mut GlGraphics) {
        let position = self.get_position();
        let rect = [
            position.x - settings::chaser::SIZE,
            position.y - settings::chaser::SIZE,
            settings::chaser::SIZE * 2.0,
            settings::chaser::SIZE * 2.0,
        ];

        ellipse(settings::color::RED, rect, c.transform, g);
    }
}
