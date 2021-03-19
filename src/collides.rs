use crate::collider::Collider;

pub trait Collides {
    fn collides_with<C: Collides>(&mut self, other: &mut C) -> bool {
        self.get_collider().collides_with(other.get_collider())
    }

    fn get_collider(&mut self) -> &mut Collider;
}
