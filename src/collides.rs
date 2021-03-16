use crate::collider::Collider;

pub trait Collides {
    fn collides_with<C: Collides>(&self, other: &mut C) -> bool;

    fn get_collider(&mut self) -> &mut Collider;
}
