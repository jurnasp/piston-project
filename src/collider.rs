use crate::settings;
use crate::vector2::Vector2;
use opengl_graphics::GlGraphics;
use piston_window::ellipse;

#[derive(Copy, Clone)]
enum ColliderState {
    Enabled,
    Disabled,
}

pub struct Collider {
    state: ColliderState,
    pub position: Vector2,
    radius: f64,
}

impl Collider {
    pub fn new(position: Vector2, radius: f64) -> Self {
        if radius <= 0.0 {
            panic!("Radius of collider must be greater than 0");
        }
        Collider {
            state: ColliderState::Enabled,
            position,
            radius,
        }
    }

    pub(crate) fn collides_with(&self, other: &Collider) -> bool {
        match (self.state, other.state) {
            (ColliderState::Enabled, ColliderState::Enabled) => {
                let min_distance = self.radius + other.radius;
                let distance = (self.position - other.position).magnitude();
                distance < min_distance
            }
            (ColliderState::Disabled, _) => false,
            (_, ColliderState::Disabled) => false,
        }
    }

    fn disable(&mut self) {
        self.state = ColliderState::Disabled
    }

    fn enable(&mut self) {
        self.state = ColliderState::Enabled
    }

    pub fn draw_debug(&self, c: piston_window::Context, g: &mut GlGraphics) {
        match self.state {
            ColliderState::Enabled => {
                let rect = [
                    self.position.x - self.radius,
                    self.position.y - self.radius,
                    self.radius * 2.0,
                    self.radius * 2.0,
                ];
                ellipse(settings::color::DEBUG, rect, c.transform, g);
            }
            ColliderState::Disabled => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collider::Collider;
    use crate::vector2::Vector2;

    #[test]
    #[should_panic]
    fn new_radius_less_than_equal_zero_panic() {
        let position = Vector2::new(0.0, 0.0);
        Collider::new(position, 0.0);
    }

    #[test]
    fn collides_with_not_colliding_with_other_returns_false() {
        let collider = Collider::new(Vector2::new(0.0, 0.0), 1.0);
        let other = Collider::new(Vector2::new(10.0, 10.0), 1.0);

        let result = collider.collides_with(&other);

        assert_eq!(result, false)
    }

    #[test]
    fn collides_with_colliding_with_other_returns_true() {
        let collider = Collider::new(Vector2::new(0.0, 0.0), 1.0);
        let other = Collider::new(Vector2::new(0.0, 0.0), 1.0);

        let result = collider.collides_with(&other);

        assert_eq!(result, true)
    }

    #[test]
    fn collides_with_colliding_with_disabled_other_returns_false() {
        let collider = Collider::new(Vector2::new(0.0, 0.0), 1.0);
        let mut other = Collider::new(Vector2::new(0.0, 0.0), 1.0);

        other.disable();
        let result = collider.collides_with(&other);

        assert_eq!(result, false)
    }

    #[test]
    fn collides_with_colliding_with_enabled_other_returns_true() {
        let collider = Collider::new(Vector2::new(0.0, 0.0), 1.0);
        let mut other = Collider::new(Vector2::new(0.0, 0.0), 1.0);

        other.disable();
        other.enable();
        let result = collider.collides_with(&other);

        assert_eq!(result, true)
    }
}
