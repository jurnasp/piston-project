use crate::vector2::Vector2;

pub trait Position {
    fn set_position(&mut self, position: Vector2);

    fn get_position(&mut self) -> Vector2;
}
