use crate::collider::Collider;
use crate::collides::Collides;
use crate::settings;
use crate::vector2::Vector2;
use opengl_graphics::GlGraphics;
use piston_window::{ellipse, Context};

pub struct Chaser {
    pub collider: Collider,
}

impl Collides for Chaser {
    fn collides_with<C: Collides>(&self, other: &C) -> bool {
        self.collider.collides_with(other.get_collider())
    }

    fn get_collider(&self) -> &Collider {
        &self.collider
    }
}

impl Chaser {
    pub(crate) fn new(position: Vector2) -> Self {
        Chaser {
            collider: Collider::new(position, settings::chaser::SIZE),
        }
    }

    pub fn update(&mut self, dt: f64, target_position: Vector2) {
        if self.collider.position == target_position {
            return;
        }

        self.collider.position = Vector2::move_towards(
            self.collider.position,
            target_position,
            settings::chaser::SPEED * dt,
        );
    }

    pub fn draw(&mut self, c: Context, g: &mut GlGraphics) {
        let rect = [
            self.collider.position.x - settings::chaser::SIZE,
            self.collider.position.y - settings::chaser::SIZE,
            settings::chaser::SIZE * 2.0,
            settings::chaser::SIZE * 2.0,
        ];

        ellipse(settings::color::RED, rect, c.transform, g);
    }
}
