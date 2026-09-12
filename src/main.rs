use macroquad::prelude::*;

const RADIUS: f32 = 30.0;

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
        speed: Vec2 { x: 1.0, y: 0.0 },
        radius: RADIUS,
        color: GREEN,
    };
}

struct Field {
    width: f32,
    height: f32,
}

struct Engine {
    gladiator: Entity,
    zombies: Vec<Entity>,
    field: Field,
}

impl Engine {
    pub fn tick(&mut self) {
        self.gladiator.position += self.gladiator.speed;
        if self.gladiator.position.x + self.gladiator.radius > self.field.width {
            self.gladiator.position.x = self.field.width - self.gladiator.radius;
        }
        if self.gladiator.position.x - self.gladiator.radius < 0.0 {
            self.gladiator.position.x = self.gladiator.radius;
        }
        if self.gladiator.position.y + self.gladiator.radius > self.field.height {
            self.gladiator.position.y = self.field.height - self.gladiator.radius;
        }
        if self.gladiator.position.y - self.gladiator.radius < 0.0 {
            self.gladiator.position.y = self.gladiator.radius;
        }
        self.gladiator.speed = Vec2::ZERO;

        for zombie in &mut self.zombies {
            zombie.position += zombie.speed;
        }
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
        zombies: vec![Entity::ZOMBIE],
        field: field,
    };

    loop {
        if is_key_down(KeyCode::W) {
            engine.gladiator.speed.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            engine.gladiator.speed.y += 1.0;
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
            engine.gladiator.position.y,
            engine.gladiator.radius,
            engine.gladiator.color,
        );

        for zombie in &engine.zombies {
            draw_circle(
                zombie.position.x,
                zombie.position.y,
                zombie.radius,
                zombie.color,
            );
        }
        next_frame().await
    }
}
