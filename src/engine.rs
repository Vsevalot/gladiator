use crate::entity::Entity;
use crate::Field;

use macroquad::prelude::*;

const MAX_ROTATION_SPEED: f32 = 0.01;
const MIN_ROTATION_SPEED: f32 = 0.001;

#[derive(Debug)]
pub struct Engine {
    pub gladiator: Entity,
    pub zombies: Vec<Entity>,
    pub field: Field,
    pub game_ended: bool,
}

impl Engine {
    fn get_position_within_field(entity: &Entity, field: &Field) -> Vec2 {
        let mut new_pos = entity.position;

        if new_pos.x + entity.radius > field.width {
            new_pos.x = field.width - entity.radius;
        }
        if new_pos.x - entity.radius < 0.0 {
            new_pos.x = entity.radius;
        }
        if new_pos.y + entity.radius > field.height {
            new_pos.y = field.height - entity.radius;
        }
        if new_pos.y - entity.radius < 0.0 {
            new_pos.y = entity.radius;
        }

        return new_pos;
    }

    pub fn new(gladiator: Entity, zombies: Vec<Entity>, field: Field) -> Self {
        let engine = Engine {
            gladiator: gladiator,
            zombies: zombies,
            field: field,
            game_ended: false,
        };
        return engine;
    }

    pub fn attack_by_gladiator(&mut self) {
        self.gladiator.attack();
    }
    pub fn trigger_attack_by_zombies(&mut self) {
        for zombie in self.zombies.iter_mut() {
            if zombie.weapon_intersects_with(&self.gladiator) {
                zombie.attack();
            }
        }
    }

    pub fn set_zombie_speed(&mut self) {
        for zombie in &mut self.zombies.iter_mut() {
            let mut angle_to_target = zombie
                .get_direction_vec()
                .angle_between(self.gladiator.position - zombie.position);

            if angle_to_target.abs() >= MAX_ROTATION_SPEED {
                if angle_to_target > 0.0 {
                    angle_to_target = MAX_ROTATION_SPEED;
                } else {
                    angle_to_target = -MAX_ROTATION_SPEED;
                }
            }
            if angle_to_target.abs() <= MIN_ROTATION_SPEED {
                // to prevent shaking and fight float
                // point operations
                angle_to_target = 0.0;
            }
            zombie.direction_angle += angle_to_target;
            zombie.speed = Vec2 {
                x: zombie.direction_angle.cos() * 0.8,
                y: zombie.direction_angle.sin() * 0.8,
            };
        }
    }

    fn remove_dead_zombies(&mut self) {
        self.zombies = self
            .zombies
            .clone()
            .into_iter()
            .filter(|z| z.hp > 0.0)
            .collect();
    }

    fn move_entitites(&mut self) {
        let mut entities = std::iter::once(&mut self.gladiator)
            .chain(self.zombies.iter_mut())
            .collect::<Vec<&mut Entity>>();

        for i in 0..entities.len() {
            let mut collided_indexes = Vec::new();
            for k in 0..entities.len() {
                if i == k {
                    continue;
                }
                if entities[i].intersects_with(entities[k]) {
                    collided_indexes.push(k);
                }
            }
            for k in collided_indexes {
                let (new_speed1, new_speed2) = get_pushed_out_speeds(entities[i], entities[k]);
                entities[i].speed += new_speed1;
                entities[k].speed += new_speed2;
            }

            let speed = entities[i].speed;
            entities[i].position += speed;
            entities[i].speed *= 0.3; // slowing down?...
            entities[i].position = Engine::get_position_within_field(entities[i], &self.field);
        }
    }

    fn apply_damage(&mut self) {
        let mut entities = std::iter::once(&mut self.gladiator)
            .chain(self.zombies.iter_mut())
            .collect::<Vec<&mut Entity>>();

        for i in 0..entities.len() {
            if !entities[i].is_attacking() {
                continue;
            }

            for k in 0..entities.len() {
                if i == k {
                    continue;
                }

                let hit_id = entities[k].id;

                if entities[i].weapon.is_already_hit_this_cycle(&hit_id) {
                    continue;
                }

                if entities[i].weapon_intersects_with(entities[k]) {
                    entities[k].hp -= entities[i].weapon.damage;
                    entities[i].weapon.register_hit(hit_id);
                }
            }
        }
    }

    fn end_game(&mut self) {
        self.game_ended = true;
    }

    pub fn tick(&mut self) {
        if self.game_ended {
            return;
        }
        self.remove_dead_zombies();
        self.set_zombie_speed();
        self.move_entitites();
        self.trigger_attack_by_zombies();
        self.apply_damage();

        let entities = std::iter::once(&mut self.gladiator)
            .chain(self.zombies.iter_mut())
            .collect::<Vec<&mut Entity>>();

        for entity in entities {
            entity.tick();
        }

        if self.gladiator.hp < 0.0 {
            self.end_game()
        }
    }

    pub fn get_entities(&mut self) -> Vec<&mut Entity> {
        return std::iter::once(&mut self.gladiator)
            .chain(self.zombies.iter_mut())
            .collect::<Vec<&mut Entity>>();
    }
}

fn get_pushed_out_speeds(entity1: &Entity, entity2: &Entity) -> (Vec2, Vec2) {
    let center_to_center_vector = entity1.position - entity2.position;
    let min_not_pushable_distance = entity1.radius + entity2.radius;
    if center_to_center_vector.length() >= min_not_pushable_distance {
        return (Vec2::ZERO, Vec2::ZERO);
    }

    let mut push_coef = min_not_pushable_distance / center_to_center_vector.length();
    push_coef = 0.01 * push_coef * push_coef * push_coef * push_coef * push_coef * push_coef;

    return (
        entity2.mass * push_coef / (entity1.mass + entity2.mass) * center_to_center_vector,
        -entity1.mass * push_coef / (entity1.mass + entity2.mass) * center_to_center_vector,
    );
}
