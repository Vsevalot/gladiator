use macroquad::prelude::*;

type ID = u32;

const RADIUS: f32 = 40.0;

#[derive(Debug, Clone)]
struct TexturePack {
    gladiator: Texture2D,
    zombie: Texture2D,
}

#[derive(Debug, Clone)]
struct Weapon {
    cooldown_ticks: u32,
    current_tick: u32,
    damage_tick_start: u32,
    damage_tick_end: u32,
    damage: f32,
    stamina_cost: f32,
    range: f32,
    damaged_this_cycle: Vec<ID>,
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
    };

    pub const BITE: Self = Self {
        cooldown_ticks: 100,
        current_tick: 0,
        damage_tick_start: 20,
        damage_tick_end: 60,
        damage: 1.0,
        stamina_cost: 10.0,
        range: 65.0,
        damaged_this_cycle: vec![],
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
struct Entity {
    id: ID,

    position: Vec2,
    speed: Vec2,
    mass: f32,
    radius: f32,
    direction_angle: f32,

    color: Color,
    texture: Texture2D,

    max_hp: f32,
    hp: f32,

    weapon: Weapon,

    max_stamina: f32,
    stamina: f32,
    stamina_recovery_per_tick: f32,
}

impl Entity {
    pub fn tick(&mut self) {
        self.weapon.tick();
        self.stamina += self.stamina_recovery_per_tick;
        if self.stamina >= self.max_stamina {
            self.stamina = self.max_stamina;
        }
    }

    pub fn make_gladiator(position: Vec2, texture: Texture2D) -> Self {
        return Self {
            id: 1,
            position: position,
            speed: Vec2::ZERO,
            radius: RADIUS,
            color: RED,
            max_hp: 100.0,
            hp: 100.0,
            direction_angle: 0.0,
            weapon: Weapon::SPEAR,
            mass: 100.0,
            max_stamina: 100.0,
            stamina: 100.0,
            stamina_recovery_per_tick: 1.0,
            texture: texture,
        };
    }

    pub fn make_zombie(id: ID, pos: Vec2, texture: Texture2D) -> Self {
        return Self {
            id: id,
            position: pos,
            speed: Vec2::ZERO,
            radius: RADIUS,
            color: GREEN,
            max_hp: 30.0,
            hp: 30.0,
            direction_angle: 0.0,
            weapon: Weapon::BITE,
            mass: 10.0,
            max_stamina: 100.0,
            stamina: 100.0,
            stamina_recovery_per_tick: 1.0,
            texture: texture,
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
    pub fn get_attack_vec(&self) -> Vec2 {
        return Vec2 {
            x: self.direction_angle.cos() * self.weapon.range,
            y: self.direction_angle.sin() * self.weapon.range,
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
    game_ended: bool,
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

    fn new(texture_pack: TexturePack, field: Field) -> Self {
        let engine = Engine {
            gladiator: Entity::make_gladiator(Vec2::ZERO, texture_pack.gladiator.clone()),
            zombies: vec![
                Entity::make_zombie(2, Vec2 { x: 200.0, y: 100.0 }, texture_pack.zombie.clone()),
                Entity::make_zombie(3, Vec2 { x: 350.0, y: 150.0 }, texture_pack.zombie.clone()),
                Entity::make_zombie(4, Vec2 { x: 500.0, y: 300.0 }, texture_pack.zombie.clone()),
            ],
            field: field,
            game_ended: false,
        };
        return engine;
    }

    fn get_next_zombie_speed_vec(zombie: &Entity, gladiator: &Entity) -> Vec2 {
        let new_speed = gladiator.position - zombie.position;
        return 0.5 * new_speed.normalize();
    }

    fn get_next_zombie_direction_angle(zombie: &Entity) -> f32 {
        return zombie.speed.y.atan2(zombie.speed.x);
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
}

fn draw_entity(entity: &Entity, field: &Field) {
    let outer_radius = entity.radius * 1.7;
    draw_rectangle(
        entity.position.x - outer_radius * 0.5,
        field.height - entity.position.y - outer_radius - 5.0,
        outer_radius,
        10.0,
        RED,
    );
    draw_rectangle(
        entity.position.x - outer_radius * 0.5,
        field.height - entity.position.y - outer_radius - 5.0,
        outer_radius * (entity.hp / entity.max_hp),
        10.0,
        GREEN,
    );

    let sprite_hw = outer_radius * 2.0;

    draw_texture_ex(
        &entity.texture,
        entity.position.x - outer_radius,
        field.height - (entity.position.y + outer_radius),
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2 {
                x: sprite_hw,
                y: sprite_hw,
            }),
            rotation: -entity.direction_angle,
            source: None,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );

    let e_attack_vec = entity.get_attack_vec();

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

fn draw_interface(hp: f32, stamina: f32) {
    draw_rectangle(10.0, 10.0, 300.0, 100.0, GRAY);
    draw_text(format!("HP: {}", hp), 20.0, 30.0, 20.0, BLACK);
    draw_text(format!("Stamina: {}", stamina), 20.0, 60.0, 20.0, BLACK);
}

fn draw_all(engine: &Engine) {
    clear_background(WHITE);
    draw_entity(&engine.gladiator, &engine.field);

    for zombie in &engine.zombies {
        draw_entity(zombie, &engine.field);
    }

    draw_interface(engine.gladiator.hp, engine.gladiator.stamina);
}

async fn load_textures() -> TexturePack {
    let gladiator_texture = load_texture("assets/textures/gladiator.png").await.unwrap();
    let zombie_texture = load_texture("assets/textures/zombie.png").await.unwrap();

    return TexturePack {
        gladiator: gladiator_texture,
        zombie: zombie_texture,
    };
}

#[macroquad::main("Gladiator")]
async fn main() {
    let field = Field {
        width: screen_width(),
        height: screen_height(),
    };

    let texture_pack = load_textures().await;

    let mut engine = Engine::new(texture_pack.clone(), field.clone());

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
            engine.attack_by_gladiator()
        }
        if is_key_down(KeyCode::R) {
            engine = Engine::new(texture_pack.clone(), field.clone());
        }
        if is_key_down(KeyCode::Q) {
            return;
        }

        draw_all(&engine);
        engine.tick();

        next_frame().await
    }
}
