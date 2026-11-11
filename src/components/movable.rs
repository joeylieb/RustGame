use macroquad::prelude::{get_frame_time, Vec2, screen_width, screen_height};
use crate::Position;

pub trait Movable {
    fn position(&self) -> &Position;
    fn position_mut(&mut self) -> &mut Position;
    fn get_step_amount(&self) -> f32;
    fn get_speed_multiplier(&self) -> f32;
    fn get_hitbox(&self) -> Vec2;
    fn move_entity(&mut self, direction: Vec2){
        let next_x = self.position().x() + direction.x * self.get_step_amount() * get_frame_time() * self.get_speed_multiplier();
        let next_y = self.position().y() + direction.y * self.get_step_amount() * get_frame_time() * self.get_speed_multiplier();

        let hitbox = self.get_hitbox();

        let clamped_x = next_x.clamp(hitbox.x / 2.0, screen_width() - hitbox.x / 2.0);
        let clamped_y = next_y.clamp(0.0, screen_height() - hitbox.y);
        self.position_mut().set_x(clamped_x);
        self.position_mut().set_y(clamped_y);
    }
}