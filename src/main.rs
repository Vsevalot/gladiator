mod animation;
mod engine;
mod entity;

use animation::Animation;
use engine::Engine;
use entity::Entity;
use macroquad::prelude::*;

use std::{f32::consts::PI, rc::Rc};

#[derive(Debug, Clone)]
struct TexturePack {
    gladiator_attack: Rc<Texture2D>,
    gladiator_walk: Rc<Texture2D>,
    gladiator_step: Rc<Texture2D>,
    zombie: Rc<Texture2D>,
}

#[derive(Debug, Clone)]
struct Field {
    width: f32,
    height: f32,
}

trait Drawable {
    fn draw(&self, field: &Field);
}

impl Drawable for Entity {
    fn draw(&self, field: &Field) {
        let outer_radius = self.radius * 1.7;
        draw_rectangle(
            self.position.x - outer_radius * 0.5,
            field.height - self.position.y - outer_radius - 5.0,
            outer_radius,
            10.0,
            RED,
        );
        draw_rectangle(
            self.position.x - outer_radius * 0.5,
            field.height - self.position.y - outer_radius - 5.0,
            outer_radius * (self.hp / self.max_hp),
            10.0,
            GREEN,
        );

        for step in self.footstep_tracker.footsteps.iter() {
            draw_texture_ex(
                &step.texture,
                step.position.x - self.radius, // - (10.0 * (-1 as i32).pow(step.should_mirror as u32) as f32),
                field.height - (step.position.y + self.radius),
                WHITE,
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

        let sprite_hw = outer_radius * 2.0;

        draw_texture_ex(
            self.walk_animation.get_texture(),
            self.position.x - outer_radius,
            field.height - (self.position.y + outer_radius),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: sprite_hw,
                    y: sprite_hw,
                }),
                rotation: -self.direction_angle - PI / 2.0,
                source: Some(self.walk_animation.get_frame()),
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        draw_texture_ex(
            self.attack_animation.get_texture(),
            self.position.x - outer_radius,
            field.height - (self.position.y + outer_radius),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: sprite_hw,
                    y: sprite_hw,
                }),
                rotation: -self.direction_angle - PI / 2.0,
                source: Some(self.attack_animation.get_frame()),
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        draw_circle(
            self.position.x,
            field.height - self.position.y,
            self.radius,
            Color::new(0.0, 0.0, 0.0, 0.3),
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
}

fn draw_interface(hp: f32, stamina: f32) {
    draw_rectangle(10.0, 10.0, 300.0, 100.0, GRAY);
    draw_text(format!("HP: {}", hp), 20.0, 30.0, 20.0, BLACK);
    draw_text(format!("Stamina: {}", stamina), 20.0, 60.0, 20.0, BLACK);
}

fn draw_all(engine: &Engine) {
    clear_background(WHITE);
    engine.gladiator.draw(&engine.field);

    for zombie in &engine.zombies {
        zombie.draw(&engine.field);
    }

    draw_interface(engine.gladiator.hp, engine.gladiator.stamina);
}

async fn load_textures() -> TexturePack {
    let walk_texture = Rc::new(load_texture("assets/textures/walking.png").await.unwrap());
    let attack_texture = Rc::new(load_texture("assets/textures/attacking.png").await.unwrap());
    let zombie_texture = Rc::new(load_texture("assets/textures/zombie.png").await.unwrap());
    let gladiator_step_texture = Rc::new(
        load_texture("assets/textures/footstep_left.png")
            .await
            .unwrap(),
    );

    return TexturePack {
        gladiator_walk: walk_texture,
        gladiator_attack: attack_texture,
        gladiator_step: gladiator_step_texture,
        zombie: zombie_texture,
    };
}

fn make_gladiator(texture_pack: &TexturePack) -> Entity {
    let mut attack_vec = vec![];
    for i in 0..7 {
        attack_vec.push((Rect::new(256.0 * (i as f32), 0.0, 256.0, 256.0), 15));
    }
    return Entity::make_gladiator(
        Vec2 { x: 50.0, y: 50.0 },
        Animation::new(
            Rc::clone(&texture_pack.gladiator_walk),
            vec![
                (Rect::new(0.0, 0.0, 256.0, 256.0), 15),
                (Rect::new(256.0, 0.0, 256.0, 256.0), 15),
                (Rect::new(512.0, 0.0, 256.0, 256.0), 15),
            ],
        ),
        Animation::new(Rc::clone(&texture_pack.gladiator_attack), attack_vec),
        Rc::clone(&texture_pack.gladiator_step),
    );
}

fn make_zombies(texture_pack: &TexturePack) -> Vec<Entity> {
    return vec![];
    // return vec![
    //     Entity::make_zombie(
    //         2,
    //         Vec2 { x: 200.0, y: 100.0 },
    //         Animation::new(
    //             Rc::clone(&texture_pack.zombie),
    //             vec![(Rect::new(0.0, 0.0, 997.0, 997.0), 15)],
    //         ),
    //     ),
    //     Entity::make_zombie(
    //         3,
    //         Vec2 { x: 350.0, y: 150.0 },
    //         Animation::new(
    //             Rc::clone(&texture_pack.zombie),
    //             vec![(Rect::new(0.0, 0.0, 997.0, 997.0), 15)],
    //         ),
    //     ),
    //     Entity::make_zombie(
    //         4,
    //         Vec2 { x: 500.0, y: 300.0 },
    //         Animation::new(
    //             Rc::clone(&texture_pack.zombie),
    //             vec![(Rect::new(0.0, 0.0, 997.0, 997.0), 15)],
    //         ),
    //     ),
    // ];
}

#[macroquad::main("Gladiator")]
async fn main() {
    let field = Field {
        width: screen_width(),
        height: screen_height(),
    };

    let texture_pack = load_textures().await;

    let mut engine = Engine::new(
        make_gladiator(&texture_pack),
        make_zombies(&texture_pack),
        field.clone(),
    );

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
            engine = Engine::new(
                make_gladiator(&texture_pack),
                make_zombies(&texture_pack),
                field.clone(),
            );
        }
        if is_key_down(KeyCode::Q) {
            return;
        }

        draw_all(&engine);

        engine.tick();

        for entity in engine.get_entities().iter_mut() {
            if entity.speed.length() > 0.1 {
                entity.walk_animation.tick();
            }
            if entity.is_attacking() {
                entity.attack_animation.tick();
            }
        }

        next_frame().await
    }
}
