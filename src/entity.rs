use std::{collections::VecDeque, f32::consts::PI, rc::Rc};

use crate::{animation::Animation, Field};
use macroquad::prelude::*;

type ID = u32;
const RADIUS: f32 = 50.0;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: ID,

    pub position: Vec2,
    pub speed: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub direction_angle: f32,

    pub walk_animation: Animation,
    pub attack_animation: Animation,

    pub max_hp: f32,
    pub hp: f32,

    pub weapon: Weapon,
    pub footstep_tracker: FootstepTracker,

    pub max_stamina: f32,
    pub stamina: f32,
    pub stamina_recovery_per_tick: f32,
}

impl Entity {
    pub fn tick(&mut self) {
        self.weapon.tick();
        self.footstep_tracker.tick();
        self.stamina += self.stamina_recovery_per_tick;
        if self.stamina >= self.max_stamina {
            self.stamina = self.max_stamina;
        }
    }

    pub fn make_gladiator(
        position: Vec2,
        walk_animation: Animation,
        attack_animation: Animation,
        step_texture: Rc<Texture2D>,
    ) -> Self {
        return Self {
            id: 1,
            position: position,
            speed: Vec2::ZERO,
            radius: RADIUS,
            max_hp: 100.0,
            hp: 100.0,
            direction_angle: 0.0,
            weapon: Weapon::SPEAR,
            footstep_tracker: FootstepTracker::new(step_texture),
            mass: 100.0,
            max_stamina: 100.0,
            stamina: 100.0,
            stamina_recovery_per_tick: 1.0,
            walk_animation: walk_animation,
            attack_animation: attack_animation,
        };
    }

    pub fn make_zombie(
        id: ID,
        pos: Vec2,
        walk_animation: Animation,
        attack_animation: Animation,
        step_texture: Rc<Texture2D>,
    ) -> Self {
        return Self {
            id: id,
            position: pos,
            speed: Vec2::ZERO,
            radius: RADIUS,
            max_hp: 30.0,
            hp: 30.0,
            direction_angle: 0.0,
            weapon: Weapon::BITE,
            footstep_tracker: FootstepTracker::new(step_texture),
            mass: 10.0,
            max_stamina: 100.0,
            stamina: 100.0,
            stamina_recovery_per_tick: 1.0,
            walk_animation: walk_animation,
            attack_animation: attack_animation,
        };
    }

    pub fn attack(&mut self) {
        if self.stamina >= self.weapon.stamina_cost {
            self.stamina -= self.weapon.stamina_cost;
            self.weapon.attack();
        }
    }
    pub fn is_attacking(&self) -> bool {
        return self.weapon.is_attacking();
    }

    pub fn weapon_intersects_with(&self, entity: &Entity) -> bool {
        if (self.get_attack_vec() - entity.position).length() < entity.radius {
            return true;
        }
        return false;
    }

    pub fn intersects_with(&self, other: &Entity) -> bool {
        let out = self.position.distance(other.position) < self.radius + other.radius;
        return out;
    }
    pub fn get_direction_vec(&self) -> Vec2 {
        return Vec2 {
            x: self.direction_angle.cos(),
            y: self.direction_angle.sin(),
        };
    }

    pub fn get_attack_vec(&self) -> Vec2 {
        return Vec2 {
            x: self.direction_angle.cos() * self.weapon.range,
            y: self.direction_angle.sin() * self.weapon.range,
        } + Vec2 {
            x: (self.direction_angle - std::f32::consts::PI / 2.0).cos() * self.weapon.offset,
            y: (self.direction_angle - std::f32::consts::PI / 2.0).sin() * self.weapon.offset,
        } + self.position;
    }

    pub fn move_self(&mut self) {
        self.position += self.speed;
        self.footstep_tracker.add_movement(
            self.speed.length(),
            self.position,
            self.direction_angle,
        );
        self.speed *= 0.3; // slowing down?...
    }

    pub fn draw_status(&self, field: &Field) {
        // HP
        draw_rectangle(
            self.position.x - self.radius * 0.5,
            field.height - self.position.y - self.radius - 15.0,
            self.radius,
            10.0,
            RED,
        );
        draw_rectangle(
            self.position.x - self.radius * 0.5,
            field.height - self.position.y - self.radius - 15.0,
            self.radius * (self.hp / self.max_hp),
            10.0,
            GREEN,
        );

        let e_direction = self.radius * self.get_direction_vec() + self.position;
        draw_line(
            self.position.x,
            field.height - self.position.y,
            e_direction.x,
            field.height - e_direction.y,
            4.0,
            BLACK,
        );

        let e_attack = self.get_attack_vec();
        let mut weapon_color = RED;
        if self.weapon.is_attacking() {
            weapon_color = ORANGE;
        }
        if self.weapon.is_damaging() {
            weapon_color = BLACK;
        }
        draw_circle(e_attack.x, field.height - e_attack.y, 5.0, weapon_color)
    }

    pub fn draw_steps(&self, field: &Field) {
        for step in self.footstep_tracker.footsteps.iter() {
            draw_texture_ex(
                &step.texture,
                step.position.x - self.radius, // - (10.0 * (-1 as i32).pow(step.should_mirror as u32) as f32),
                field.height - (step.position.y + self.radius),
                Color {
                    r: (1.0),
                    g: (1.0),
                    b: (1.0),
                    a: (step.opacity),
                },
                DrawTextureParams {
                    dest_size: None,
                    rotation: -step.direction_angle + PI / 2.0,
                    source: None,
                    flip_x: step.should_mirror,
                    flip_y: false,
                    pivot: None,
                },
            );
        }
    }

    pub fn draw_hitbox(&self, field: &Field) {
        draw_circle(
            self.position.x,
            field.height - self.position.y,
            self.radius,
            Color::new(0.0, 0.0, 0.0, 0.3),
        );
    }
}

#[derive(Debug, Clone)]
pub struct Weapon {
    cooldown_ticks: u32,
    current_tick: u32,
    damage_tick_start: u32,
    damage_tick_end: u32,
    pub damage: f32,
    stamina_cost: f32,
    range: f32,
    damaged_this_cycle: Vec<ID>,
    offset: f32,
}

impl Weapon {
    pub const SPEAR: Self = Self {
        cooldown_ticks: 100 * 15,
        current_tick: 0,
        damage_tick_start: 100 * 8,
        damage_tick_end: 100 * 12,
        damage: 10.0,
        stamina_cost: 10.0,
        range: 65.0,
        damaged_this_cycle: vec![],
        offset: RADIUS * 0.6,
    };

    pub const BITE: Self = Self {
        cooldown_ticks: 100,
        current_tick: 0,
        damage_tick_start: 20,
        damage_tick_end: 60,
        damage: 8.0,
        stamina_cost: 10.0,
        range: 65.0,
        damaged_this_cycle: vec![],
        offset: 0.0,
    };

    pub fn can_attack(&self) -> bool {
        return self.current_tick == 0;
    }

    pub fn is_attacking(&self) -> bool {
        return self.current_tick != 0;
    }

    pub fn is_damaging(&self) -> bool {
        return self.damage_tick_start <= self.current_tick
            && self.current_tick <= self.damage_tick_end;
    }

    pub fn register_hit(&mut self, entity_id: ID) {
        self.damaged_this_cycle.push(entity_id);
    }

    pub fn is_already_hit_this_cycle(&self, entity_id: &ID) -> bool {
        return self.damaged_this_cycle.contains(entity_id);
    }
    pub fn attack(&mut self) {
        if self.can_attack() {
            self.current_tick = 1;
        }
    }

    pub fn tick(&mut self) {
        if self.is_attacking() {
            self.current_tick += 1;
            if self.current_tick >= self.cooldown_ticks {
                self.current_tick = 0;
                self.damaged_this_cycle.clear();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Footstep {
    pub texture: Rc<Texture2D>,
    pub position: Vec2,
    pub direction_angle: f32,
    pub should_mirror: bool,
    pub opacity: f32,
}

#[derive(Debug, Clone)]
pub struct FootstepTracker {
    distance_since_last: f32,
    current_direction_angle: f32,
    current_position: Vec2,
    distance_per_step: f32,
    pub footsteps: VecDeque<Footstep>,
    texture: Rc<Texture2D>,
}

impl FootstepTracker {
    fn new(texture: Rc<Texture2D>) -> Self {
        return Self {
            distance_since_last: 0.0,
            current_direction_angle: 0.0,
            current_position: Vec2::ZERO,
            distance_per_step: 20.0,
            footsteps: VecDeque::new(),
            texture: texture,
        };
    }

    fn add_movement(&mut self, distance: f32, position: Vec2, direction_angle: f32) {
        self.distance_since_last += distance;
        self.current_direction_angle = direction_angle;
        self.current_position = position;
    }

    fn tick(&mut self) {
        if self.distance_since_last >= self.distance_per_step {
            let mut should_mirror = false;
            if let Some(f) = self.footsteps.back() {
                should_mirror = !f.should_mirror;
            }
            self.footsteps.push_back(Footstep {
                texture: Rc::clone(&self.texture),
                direction_angle: self.current_direction_angle,
                position: self.current_position,
                should_mirror: should_mirror,
                opacity: 1.0,
            });

            if self.footsteps.len() > 20 {
                // my magic max number of steps
                self.footsteps.pop_front();
            }
            for step in self.footsteps.iter_mut() {
                step.opacity -= 0.05
            }
            self.distance_since_last = 0.0;
        }
    }
}
