use macroquad::prelude::*;

const RADIUS: f32 = 30.0;
const ATTACK_RANGE: f32 = 45.0;

#[derive(Debug, Clone, Copy)]
struct Weapon {
    cooldown_ticks: u32,
    current_tick: u32,
    damage_tick_start: u32,
    damage_tick_end: u32,
    damage: f32,
}

impl Weapon {
    pub const SPEAR: Self = Self {
        cooldown_ticks: 500,
        current_tick: 0,
        damage_tick_start: 100,
        damage_tick_end: 200,
        damage: 10.0,
    };
    pub const BITE: Self = Self {
        cooldown_ticks: 100,
        current_tick: 0,
        damage_tick_start: 20,
        damage_tick_end: 60,
        damage: 10.0,
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
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Entity {
    position: Vec2,
    speed: Vec2,
    radius: f32,
    color: Color,
    hp: f32,
    direction_angle: f32,
    weapon: Weapon,
    mass: f32,
}

impl Entity {
    pub const GLADIATOR: Self = Self {
        position: Vec2::ZERO,
        speed: Vec2::ZERO,
        radius: RADIUS,
        color: RED,
        hp: 100.0,
        direction_angle: 0.0,
        weapon: Weapon::SPEAR,
        mass: 100.0,
    };
    pub const ZOMBIE: Self = Self {
        position: Vec2 { x: 300.0, y: 100.0 },
        speed: Vec2::ZERO,
        radius: RADIUS,
        color: GREEN,
        hp: 30.0,
        direction_angle: 0.0,
        weapon: Weapon::BITE,
        mass: 10.0,
    };
    pub fn tick(&mut self) {
        self.weapon.tick();
    }
    pub fn make_zombie(pos: Vec2) -> Self {
        let mut z = Self::ZOMBIE;
        z.position = pos;
        return z;
    }

    pub fn attack(&mut self) {
        self.weapon.attack();
    }

    pub fn intersects_with(&self, other: &Entity) -> bool {
        let out = self.position.distance(other.position) < self.radius + other.radius;
        return out;
    }
    pub fn get_attack_vec2(&self) -> Vec2 {
        return Vec2 {
            x: self.direction_angle.cos() * ATTACK_RANGE,
            y: self.direction_angle.sin() * ATTACK_RANGE,
        } + self.position;
    }
}

#[derive(Debug, Clone)]
struct Field {
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct Engine {
    gladiator: Entity,
    zombies: Vec<Entity>,
    field: Field,
}

fn get_new_speeds(entity1: &Entity, entity2: &Entity) -> (Vec2, Vec2) {
    let v1 = (2.0 * entity2.mass * entity2.speed + (entity1.mass - entity2.mass) * entity1.speed)
        / (entity1.mass + entity2.mass);
    let v2 = (2.0 * entity1.mass * entity1.speed + (entity2.mass - entity1.mass) * entity2.speed)
        / (entity1.mass + entity2.mass);
    return (v1, -v2);
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

    fn new(field: Field) -> Self {
        let engine = Engine {
            gladiator: Entity::GLADIATOR,
            zombies: vec![
                Entity::make_zombie(Vec2 { x: 200.0, y: 100.0 }),
                Entity::make_zombie(Vec2 { x: 350.0, y: 150.0 }),
                Entity::make_zombie(Vec2 { x: 500.0, y: 300.0 }),
            ],
            field: field,
        };
        return engine;
    }

    fn get_next_zombie_speed_vec(zombie: &Entity, gladiator: &Entity) -> Vec2 {
        let new_speed = gladiator.position - zombie.position;
        return 0.3 * new_speed.normalize();
        // return Vec2::ZERO;
    }

    fn get_next_zombie_direction_angle(zombie: &Entity) -> f32 {
        return zombie.speed.y.atan2(zombie.speed.x);
    }

    pub fn attack(&mut self) {
        self.gladiator.attack();
    }

    pub fn set_zombie_speed(&mut self) {
        for zombie in &mut self.zombies.iter_mut() {
            zombie.speed = Engine::get_next_zombie_speed_vec(zombie, &self.gladiator);
            zombie.direction_angle = Engine::get_next_zombie_direction_angle(zombie);
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
                if i == k {continue;}
                if entities[i].intersects_with(entities[k]) {
                    collided_indexes.push(k);
                }
            }
            println!("Collided: {:?}", collided_indexes);
            for k in collided_indexes {
                let (new_speed1, new_speed2) = get_new_speeds(entities[i], entities[k]);
                entities[i].speed = new_speed1;
                entities[k].speed = new_speed2;
            }

            let speed = entities[i].speed;
            entities[i].position += speed;
            entities[i].speed = Vec2::ZERO;
            entities[i].position = Engine::get_position_within_field(entities[i], &self.field);
        }
    }

    pub fn tick(&mut self) {
        if self.gladiator.weapon.is_damaging() {
            let damage_point = self.gladiator.get_attack_vec2();
            for zombie in self.zombies.iter_mut() {
                if zombie.position.distance(damage_point) < zombie.radius {
                    zombie.hp -= self.gladiator.weapon.damage;
                }
            }
        }
        self.remove_dead_zombies();
        self.set_zombie_speed();
        self.move_entitites();
        self.gladiator.tick()
    }
}

fn draw_entity(entity: &Entity, field: &Field) {
    draw_circle(
        entity.position.x,
        field.height - entity.position.y,
        entity.radius,
        entity.color,
    );

    let e_attack_vec = entity.get_attack_vec2();

    let mut weapon_color = YELLOW;
    if entity.weapon.is_attacking() {
        weapon_color = ORANGE;
    }
    if entity.weapon.is_damaging() {
        weapon_color = BLACK;
    }
    draw_line(
        entity.position.x,
        field.height - entity.position.y,
        e_attack_vec.x,
        field.height - e_attack_vec.y,
        4.0,
        weapon_color,
    );
}

#[macroquad::main("Gladiator")]
async fn main() {
    let field = Field {
        width: screen_width(),
        height: screen_height(),
    };
    let mut engine = Engine::new(field.clone());

    loop {
        if is_key_down(KeyCode::W) {
            engine.gladiator.speed.y += 1.0;
        }
        if is_key_down(KeyCode::S) {
            engine.gladiator.speed.y -= 1.0;
        }
        if is_key_down(KeyCode::A) {
            engine.gladiator.speed.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            engine.gladiator.speed.x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            engine.gladiator.direction_angle -= (0.1) / std::f32::consts::PI;
        }
        if is_key_down(KeyCode::Right) {
            engine.gladiator.direction_angle += (0.1) / std::f32::consts::PI;
        }
        if is_key_down(KeyCode::Space) {
            engine.attack()
        }
        if is_key_down(KeyCode::R) {
            engine = Engine::new(field.clone());
        }
        if is_key_down(KeyCode::Q) {
            return;
        }

        engine.tick();

        clear_background(WHITE);
        draw_entity(&engine.gladiator, &engine.field);

        for zombie in &engine.zombies {
            draw_entity(zombie, &engine.field);
        }
        next_frame().await
    }
}
