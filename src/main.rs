use macroquad::prelude::*;

const RADIUS: f32 = 30.0;
const ATTACK_RANGE: f32 = 45.0;

#[derive(Debug, Clone, Copy)]
struct Entity {
    position: Vec2,
    speed: Vec2,
    radius: f32,
    color: Color,
    hp: f32,
    direction_angle: f32,
}

impl Entity {
    pub const GLADIATOR: Self = Self {
        position: Vec2::ZERO,
        speed: Vec2::ZERO,
        radius: RADIUS,
        color: RED,
        hp: 100.0,
        direction_angle: 0.0,
    };
    pub const ZOMBIE: Self = Self {
        position: Vec2 { x: 300.0, y: 100.0 },
        speed: Vec2 { x: -1.0, y: 0.0 },
        radius: RADIUS,
        color: GREEN,
        hp: 30.0,
        direction_angle: 0.0,
    };
    pub fn make_zombie(pos: Vec2) -> Self {
        let mut z = Self::ZOMBIE;
        z.position = pos;
        return z;
    }

    pub fn intersects_with(&self, other: &Entity) -> bool {
        let out = self.position.distance(other.position) < self.radius + other.radius;
        return out;
    }
    pub fn get_attack_vec2(&self) -> Vec2 {
        return Vec2{
            x: self.direction_angle.cos() * ATTACK_RANGE,
            y: self.direction_angle.sin() * ATTACK_RANGE,
        } + self.position;
    }
}

#[derive(Debug)]
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

    fn get_next_zombie_speed_vec(zombie: &Entity, gladiator: &Entity) -> Vec2 {
        let new_speed = gladiator.position - zombie.position;
        return 0.2*new_speed.normalize();
    }

    fn get_next_zombie_direction_angle(zombie: &Entity) -> f32 {
        return zombie.speed.y.atan2(zombie.speed.x);
    }

    pub fn move_gladiator(&mut self) {
        let old_pos = self.gladiator.position;
        self.gladiator.position += self.gladiator.speed;
        self.gladiator.position = Engine::get_position_within_field(&self.gladiator, &self.field);

        if self.zombies
           .iter()
           .any(|z| self.gladiator.intersects_with(&z))
        {
            self.gladiator.position = old_pos;
        }

        self.gladiator.speed = Vec2::ZERO;
    }

    pub fn move_zombies(&mut self) {
        let zombies_copy = self.zombies.clone();

        for (i, zombie) in &mut self.zombies.iter_mut().enumerate() {
            let old_pos = zombie.position;
            zombie.speed = Engine::get_next_zombie_speed_vec(zombie, &self.gladiator);
            zombie.position += zombie.speed;
            zombie.direction_angle = Engine::get_next_zombie_direction_angle(zombie);
            zombie.position = Engine::get_position_within_field(zombie, &self.field);

            if zombies_copy
               .iter()
               .enumerate()
               .any(|(k, z)| { (k != i) && zombie.intersects_with(&z)})
               || self.gladiator.intersects_with(zombie)
            {
                zombie.position = old_pos;
            }
        }
    }

    pub fn tick(&mut self) {
        self.move_gladiator();
        self.move_zombies();
    }
}

fn draw_entity(entity: &Entity, field: &Field){
    draw_circle(
        entity.position.x,
        field.height-entity.position.y,
        entity.radius,
        entity.color,
    );

    let e_attack_vec = entity.get_attack_vec2();

    draw_line(
        entity.position.x,
        field.height-entity.position.y,
        e_attack_vec.x,
        field.height-e_attack_vec.y,
        4.0,
        ORANGE,
    );
}

#[macroquad::main("Gladiator")]
async fn main() {
    let field = Field {
        width: screen_width(),
        height: screen_height(),
    };
    let mut engine = Engine {
        gladiator: Entity::GLADIATOR,
        zombies: vec![
            Entity::make_zombie(Vec2{x: 200.0, y: 100.0}),
            Entity::make_zombie(Vec2{x: 350.0, y: 150.0}),
            Entity::make_zombie(Vec2{x: 500.0, y: 300.0}),
        ],
        field: field,
    };

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
            engine.gladiator.direction_angle -= (0.1)/3.14;
        }
        if is_key_down(KeyCode::Right) {
            engine.gladiator.direction_angle += (0.1)/3.14;
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
