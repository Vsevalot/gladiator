use macroquad::prelude::*;

const RADIUS: f32 = 30.0;

#[derive(Debug, Clone, Copy)]
struct Entity {
    position: Vec2,
    speed: Vec2,
    radius: f32,
    color: Color,
}

impl Entity {
    pub const GLADIATOR: Self = Self {
        position: Vec2::ZERO,
        speed: Vec2::ZERO,
        radius: RADIUS,
        color: RED,
    };
    pub const ZOMBIE: Self = Self {
        position: Vec2 { x: 300.0, y: 100.0 },
        speed: Vec2 { x: -1.0, y: 0.0 },
        radius: RADIUS,
        color: GREEN,
    };
    pub fn make_zombie(pos: Vec2) -> Self {
        let mut z = Self::ZOMBIE;
        z.position = pos;
        return z;
    }

    pub fn distance_to(&self, other: &Entity) -> f32 {
        return (
            (self.position.x - other.position.x) * (self.position.x - other.position.x) + 
            (self.position.y - other.position.y) * (self.position.y - other.position.y)
        ).sqrt()
    }

    pub fn intersects_with(&self, other: &Entity) -> bool {
        let out = self.distance_to(other) < self.radius + other.radius;
        return out;
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
        return 0.2*(new_speed / new_speed.length());
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
            zombie.position += Engine::get_next_zombie_speed_vec(zombie, &self.gladiator);
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
        if is_key_down(KeyCode::Q) {
            return;
        }

        engine.tick();

        clear_background(WHITE);
        draw_circle(
            engine.gladiator.position.x,
            engine.field.height-engine.gladiator.position.y,
            engine.gladiator.radius,
            engine.gladiator.color,
        );

        for zombie in &engine.zombies {
            draw_circle(
                zombie.position.x,
                engine.field.height-zombie.position.y,
                zombie.radius,
                zombie.color,
            );
        }
        next_frame().await
    }
}
