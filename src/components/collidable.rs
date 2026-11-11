use macroquad::math::{Rect, Vec2};

pub trait Collidable {
    fn get_collide_hitbox(&self) -> Rect;
    fn collided_with<T: Collidable>(&self, other: &T) -> bool {
        self.get_collide_hitbox().overlaps(&other.get_collide_hitbox())
    }
}

pub fn check_collision<A: Collidable, B: Collidable>(a: &A, b: &B) -> bool {
    a.get_collide_hitbox().overlaps(&b.get_collide_hitbox())
}