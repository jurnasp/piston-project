use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Default, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    pub fn move_towards(position: Self, target: Self, max_distance_delta: f64) -> Self {
        let position_diff = target - position;
        let magnitude = position_diff.magnitude();

        if magnitude <= max_distance_delta || magnitude == 0.0 {
            return target;
        }

        position_diff / magnitude * max_distance_delta + position
    }

    fn magnitude(self: Self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Div<f64> for Vector2 {
    type Output = Self;
    fn div(self, a: f64) -> Self {
        let x = self.x / a;
        let y = self.y / a;

        Vector2 { x, y }
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Self;
    fn div(self, a: Self) -> Self {
        let x = self.x / a.x;
        let y = self.y / a.y;

        Vector2 { x, y }
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Self;
    fn mul(self, a: Self) -> Self {
        let x = self.x * a.x;
        let y = self.y * a.y;

        Vector2 { x, y }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Self;
    fn mul(self, a: f64) -> Self {
        let x = self.x * a;
        let y = self.y * a;

        Vector2 { x, y }
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Self;
    fn sub(self, a: Self) -> Self {
        let x = self.x - a.x;
        let y = self.y - a.y;

        Vector2 { x, y }
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Self;
    fn add(self, a: Self) -> Self {
        let x = self.x + a.x;
        let y = self.y + a.y;

        Vector2 { x, y }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector2::Vector2;

    #[test]
    fn move_towards_horizontal_target_position_moves_to_target_position() {
        let start = Vector2::new(0.0, 0.0);
        let target = Vector2::new(1.0, 0.0);

        let result = Vector2::move_towards(start, target, 2.0);

        assert_eq!(result, Vector2::new(1.0, 0.0));
    }

    #[test]
    fn move_towards_horizontal_capped_by_max_distance_delta_moves_to_max_distance_delta() {
        let start = Vector2::new(0.0, 0.0);
        let target = Vector2::new(1.0, 0.0);

        let result = Vector2::move_towards(start, target, 0.5);

        assert_eq!(result, Vector2::new(0.5, 0.0));
    }

    #[test]
    fn move_towards_vertical_target_position_moves_to_target_position() {
        let start = Vector2::new(0.0, 0.0);
        let target = Vector2::new(0.0, 1.0);

        let result = Vector2::move_towards(start, target, 2.0);

        assert_eq!(result, Vector2::new(0.0, 1.0));
    }

    #[test]
    fn move_towards_vertical_capped_by_max_distance_delta_moves_to_max_distance_delta() {
        let start = Vector2::new(0.0, 0.0);
        let target = Vector2::new(0.0, 1.0);

        let result = Vector2::move_towards(start, target, 0.5);

        assert_eq!(result, Vector2::new(0.0, 0.5));
    }

    #[test]
    fn move_towards_diagonal_target_position_moves_to_target_position() {
        let start = Vector2::new(0.0, 0.0);
        let target = Vector2::new(1.0, 1.0);

        let result = Vector2::move_towards(start, target, 2.0);

        assert_eq!(result, Vector2::new(1.0, 1.0));
    }

    #[test]
    fn move_towards_diagonal_target_moves_to_target_position() {
        let start = Vector2::new(0.0, 0.0);
        let target = Vector2::new(3.0, 4.0);

        let end = Vector2::move_towards(start, target, 5.0);

        assert_eq!(end, Vector2::new(3.0, 4.0));
    }

    #[test]
    fn move_towards_diagonal_target_capped_by_max_distance_delta_moves_to_max_distance_delta() {
        let start = Vector2::new(0.0, 0.0);
        let target = Vector2::new(30.0, 40.0);

        let end = Vector2::move_towards(start, target, 5.0);

        assert_eq!(end, Vector2::new(3.0, 4.0));
    }

    #[test]
    fn partial_eq_position_by_equal_position() {
        let position = Vector2::new(2.0, 2.0);
        let equal_position = Vector2::new(2.0, 2.0);

        let result = position == equal_position;

        assert_eq!(result, true);
    }

    #[test]
    fn partial_eq_position_by_non_equal_position() {
        let position = Vector2::new(2.0, 2.0);
        let non_equal_position = Vector2::new(0.0, 0.0);

        let result = position == non_equal_position;

        assert_eq!(result, false);
    }

    #[test]
    fn div_position_by_pos_f64() {
        let position = Vector2::new(2.0, 2.0);
        let pos_f64 = 2.0;

        let result = position / pos_f64;

        assert_eq!(result, Vector2::new(1.0, 1.0));
    }

    #[test]
    fn div_position_by_pos_position() {
        let position = Vector2::new(2.0, 2.0);
        let pos_position = Vector2::new(2.0, 2.0);

        let result = position / pos_position;

        assert_eq!(result, Vector2::new(1.0, 1.0));
    }

    #[test]
    fn mul_position_by_pos_f64() {
        let position = Vector2::new(2.0, 2.0);
        let pos_f64 = 2.0;

        let result = position * pos_f64;

        assert_eq!(result, Vector2::new(4.0, 4.0));
    }

    #[test]
    fn mul_position_by_pos_position() {
        let position = Vector2::new(2.0, 2.0);
        let pos_position = Vector2::new(2.0, 2.0);

        let result = position * pos_position;

        assert_eq!(result, Vector2::new(4.0, 4.0));
    }

    #[test]
    fn sub_position_with_pos_position() {
        let position = Vector2::new(2.0, 2.0);
        let pos_position = Vector2::new(2.0, 2.0);

        let result = position - pos_position;

        assert_eq!(result, Vector2::new(0.0, 0.0));
    }

    #[test]
    fn sub_position_with_neg_position() {
        let position = Vector2::new(2.0, 2.0);
        let neg_position = Vector2::new(-2.0, -2.0);

        let result = position - neg_position;

        assert_eq!(result, Vector2::new(4.0, 4.0));
    }

    #[test]
    fn add_position_to_pos_position() {
        let position = Vector2::new(2.0, 2.0);
        let pos_position = Vector2::new(2.0, 2.0);

        let result = position + pos_position;

        assert_eq!(result, Vector2::new(4.0, 4.0));
    }

    #[test]
    fn add_position_to_neg_position() {
        let position = Vector2::new(2.0, 2.0);
        let neg_position = Vector2::new(-2.0, -2.0);

        let result = position + neg_position;

        assert_eq!(result, Vector2::new(0.0, 0.0));
    }
}
