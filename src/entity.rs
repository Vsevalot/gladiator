use crate::animation::Animation;
use macroquad::prelude::*;

type ID = u32;
const RADIUS: f32 = 40.0;

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: ID,

    pub position: Vec2,
    pub speed: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub direction_angle: f32,

    pub animation: Animation,

    pub max_hp: f32,
    pub hp: f32,

    pub weapon: Weapon,

    pub max_stamina: f32,
    pub stamina: f32,
    pub stamina_recovery_per_tick: f32,
}

impl Entity {
    pub fn tick(&mut self) {
        self.weapon.tick();
        self.stamina += self.stamina_recovery_per_tick;
        if self.stamina >= self.max_stamina {
            self.stamina = self.max_stamina;
        }
    }

    pub fn make_gladiator(position: Vec2, animation: Animation) -> Self {
        return Self {
            id: 1,
            position: position,
            speed: Vec2::ZERO,
            radius: RADIUS,
            max_hp: 100.0,
            hp: 100.0,
            direction_angle: 0.0,
            weapon: Weapon::SPEAR,
            mass: 100.0,
            max_stamina: 100.0,
            stamina: 100.0,
            stamina_recovery_per_tick: 1.0,
            animation: animation,
        };
    }

    pub fn make_zombie(id: ID, pos: Vec2, animation: Animation) -> Self {
        return Self {
            id: id,
            position: pos,
            speed: Vec2::ZERO,
            radius: RADIUS,
            max_hp: 30.0,
            hp: 30.0,
            direction_angle: 0.0,
            weapon: Weapon::BITE,
            mass: 10.0,
            max_stamina: 100.0,
            stamina: 100.0,
            stamina_recovery_per_tick: 1.0,
            animation: animation,
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
        cooldown_ticks: 50,
        current_tick: 0,
        damage_tick_start: 10,
        damage_tick_end: 20,
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
