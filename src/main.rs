pub mod components;

use crate::components::position::Position;
use crate::components::movable::Movable;

use std::cmp::PartialEq;
use macroquad::prelude::*;
use crate::components::collidable::{check_collision, Collidable};
use crate::LifeState::{Alive, Dead};

struct GlobalOptions {
    view_hitboxes: bool
}

#[derive(PartialEq)]
enum LifeState {
    Alive,
    Dead
}

struct Character {
    health: f32,
    position: Position,
    step_px: f32,
    speed_multiplier: f32,
    life_state: LifeState
}

impl Character {
    fn new(position: Position) -> Self {
        Character {
            health: 100.0,
            position,
            step_px: 200.0,
            speed_multiplier: 1.0,
            life_state: LifeState::Alive
        }
    }
    fn draw_character(&self, global_options: &GlobalOptions) {
        if global_options.view_hitboxes {
            draw_rectangle_lines(self.position.x() - 20.0, self.position.y(), self.get_hitbox().x, self.get_hitbox().y, 5.0, WHITE);
        }

        draw_circle(self.position.x(), self.position.y() + 20.0, 20.0, BLUE);
        draw_line(self.position.x(), self.position.y() + 35.0, self.position.x(), self.position.y() + 80.0, 10.0, BLUE);
        draw_line(self.position.x(), self.position.y() + 80.0, self.position.x() - 10.0, self.position.y() + 120.0, 10.0, BLUE);
        draw_line(self.position.x(), self.position.y() + 80.0, self.position.x() + 10.0, self.position.y() + 120.0, 10.0, BLUE);
    }
    fn set_sprint(&mut self, state: bool){
        if state {
            self.speed_multiplier = 1.5;
        } else {
            self.speed_multiplier = 1.0;
        }
    }
    fn damage(&mut self, amount: f32){
        if self.health - amount <= 0.0 {
            self.life_state = Dead
        }
        self.health -= amount
    }
}

impl Movable for Character {
    fn position(&self) -> &Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }

    fn get_step_amount(&self) -> f32 {
        self.step_px
    }

    fn get_speed_multiplier(&self) -> f32 {
        self.speed_multiplier
    }
    fn get_hitbox(&self) -> Vec2 {
        Vec2::new(40.0, 120.0)
    }
}

impl Collidable for Character {
    fn get_collide_hitbox(&self) -> Rect {
        Rect::new(self.position.x() - 20.0, self.position.y(), self.get_hitbox().x, self.get_hitbox().y)
    }
}

#[derive(PartialEq)]
enum EnemyState {
    Homing,
    Dead,
    PowerUp,
    Spawning
}

struct Enemy {
    position: Position,
    color: Color,
    step_px: f32,
    state: EnemyState,
    spawn_timer: f32
}

impl Enemy {
    fn new(position: Position) -> Self {
        Enemy {
            position,
            color: GREEN,
            step_px: 100.0,
            state: EnemyState::Spawning,
            spawn_timer: 0.0
        }
    }
    fn draw_enemy(&self, global_options: &GlobalOptions) {
        if global_options.view_hitboxes {
            draw_rectangle_lines(self.position.x() - 10.0, self.position.y() - 10.0, self.get_hitbox().x, self.get_hitbox().y, 5.0, WHITE);
        }

        draw_circle(self.position.x(), self.position.y(), 10.0, self.color);
    }
    fn attack_character(&mut self, character: &mut Character){
        if self.state == EnemyState::Spawning {
            self.spawn_timer += get_frame_time();
            let color_start = Color::from_rgba(150, 150, 150, 77);
            let color_end = Color::from_rgba(2, 210, 102, 173);
            self.color = Color::from_vec(color_start.to_vec().lerp(color_end.to_vec(), (self.spawn_timer/2.0).clamp(0.0,1.0)));
            if self.spawn_timer >= 2.0 {
                self.state = EnemyState::Homing;
            }
            return;
        }

        let player_direction: Vec2 = Vec2::new(character.position.x() - self.position.x(), character.position.y() - self.position.y());
        if self.collided_with(character) {
            self.collide();
            character.damage(10.0);
            return;
        }
        self.move_entity(player_direction.normalize_or_zero());
    }
    fn collide(&mut self){
        self.state = EnemyState::Dead
    }
}

impl Movable for Enemy {
    fn position(&self) -> &Position {
        &self.position
    }
    fn position_mut(&mut self) -> &mut Position {
        &mut self.position
    }
    fn get_step_amount(&self) -> f32 {
        self.step_px
    }
    fn get_speed_multiplier(&self) -> f32 {
        1.0
    }
    fn get_hitbox(&self) -> Vec2 {
        Vec2::new(20.0, 20.0)
    }
}

impl Collidable for Enemy {
    fn get_collide_hitbox(&self) -> Rect {
        Rect::new(self.position.x() - 10.0, self.position.y() - 10.0, self.get_hitbox().x, self.get_hitbox().y)
    }
}

fn handle_inputs(character: &mut Character, options: &mut GlobalOptions) {
    let mut movement_vector = Vec2::ZERO;

    if is_key_down(KeyCode::W){
        movement_vector.y -= 1.0;
    }
    if is_key_down(KeyCode::A){
        movement_vector.x -= 1.0;
    }

    if is_key_down(KeyCode::S){
        movement_vector.y += 1.0;
    }

    if is_key_down(KeyCode::D){
        movement_vector.x += 1.0;
    }

    if is_key_pressed(KeyCode::H) {
        options.view_hitboxes = !options.view_hitboxes;
    }

    character.set_sprint(is_key_down(KeyCode::LeftShift));
    character.move_entity(movement_vector.normalize_or_zero());
}


#[macroquad::main("BasicGame")]
async fn main() {
    let mut character = Character::new(Position::new(300.0,300.0));
    let mut options: GlobalOptions = GlobalOptions {
        view_hitboxes: false
    };
    let mut enemies: Vec<Enemy> = Vec::new();

    loop {
        if character.life_state == Dead {
            clear_background(GRAY);
            draw_text("You Died.", screen_width()/2.0, screen_height()/2.0, 24.0, WHITE );
            next_frame().await;
            continue
        }

        clear_background(RED);
        handle_inputs(&mut character, &mut options);

        draw_text(&format!("Health: {:.0}", character.health), screen_width() - 150.0, 30.0, 24.0, WHITE);

        character.draw_character(&options);

        spawn_enemy(&mut enemies);

        for enemy in &mut enemies {
            enemy.attack_character(&mut character);
            enemy.draw_enemy(&options);
        }

        enemies.retain(|enemy| enemy.state != EnemyState::Dead);

        next_frame().await;
    }
}

fn spawn_enemy(enemies: &mut Vec<Enemy>) {
    let dt = get_frame_time();
    let probability = 1.0 - (-0.2 * dt).exp();

    if rand::gen_range(0.0, 1.0) < probability {
        let bounds = Position::new(rand::gen_range(5.0, screen_width() - 5.0), rand::gen_range(5.0, screen_height() - 5.0));
        let spawned_enemy = Enemy::new(bounds);
        enemies.push(spawned_enemy);
    }
}
