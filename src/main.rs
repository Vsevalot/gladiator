mod animation;
mod engine;
mod entity;
mod serialization;

use animation::Animation;
use engine::Engine;
use entity::{Entity, TICKS_PER_FRAME};
use macroquad::prelude::*;

pub const SPEED_TO_ANIMATE_WALK: f32 = 0.3;

use std::{collections::HashMap, f32::consts::PI, rc::Rc};

use crate::{
    animation::ProtoAnimation,
    entity::FootstepTracker,
    serialization::{MyDeserialize, MySerialize},
};

struct AnimationPack {
    gladiator_attack: ProtoAnimation,
    gladiator_walk: ProtoAnimation,
    gladiator_step_texture: Rc<Texture2D>,
    zombie: ProtoAnimation,
}

#[derive(Debug, Clone)]
struct Field {
    width: f32,
    height: f32,
}

impl MySerialize for Field {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        self.width.serialize(buffer);
        self.height.serialize(buffer);
    }
}

impl MyDeserialize for Field {
    fn deserialize(buffer: &[u8], cursor: &mut usize) -> Result<Self, String> {
        Ok(Field {
            width: f32::deserialize(buffer, cursor)?,
            height: f32::deserialize(buffer, cursor)?,
        })
    }
}

fn draw_interface(hp: f32, stamina: f32) {
    draw_rectangle(10.0, 10.0, 300.0, 100.0, GRAY);
    draw_text(format!("HP: {}", hp), 20.0, 30.0, 20.0, BLACK);
    draw_text(format!("Stamina: {}", stamina), 20.0, 60.0, 20.0, BLACK);
    draw_text(
        format!("Frame time: {}ms", get_frame_time() * 1000.0),
        20.0,
        90.0,
        20.0,
        BLACK,
    );
}

fn draw_all(
    engine: &Engine,
    entity_to_attack_animation: &mut HashMap<u32, Animation>,
    entity_to_walking_animation: &mut HashMap<u32, Animation>,
    entity_to_footstep_tracker: &mut HashMap<u32, FootstepTracker>,
) {
    clear_background(WHITE);

    for entity in engine.get_entities() {
        let attack_animation = entity_to_attack_animation.get_mut(&entity.id).unwrap();
        let walk_animation = entity_to_walking_animation.get_mut(&entity.id).unwrap();
        let footstep_tracker = entity_to_footstep_tracker.get_mut(&entity.id).unwrap();
        draw_entity(
            entity,
            &engine.field,
            engine.global_tick,
            attack_animation,
            walk_animation,
            footstep_tracker,
        );
    }

    draw_interface(engine.gladiator.hp, engine.gladiator.stamina);
}

fn draw_entity(
    entity: &Entity,
    field: &Field,
    global_tick: u32,
    attack_animation: &mut Animation,
    walk_animation: &mut Animation,
    footstep_tracker: &mut FootstepTracker,
) {
    entity.draw_status(field);
    entity.draw_hitbox(field);
    draw_steps(field, global_tick, footstep_tracker);

    if entity.is_attacking() {
        let texture_sector = attack_animation.get_frame();
        let mut frame = attack_animation.get_frame();
        frame.scale(2.0, 2.0);
        draw_texture_ex(
            attack_animation.get_texture(),
            entity.position.x - frame.h / 2.0,
            field.height - (entity.position.y + frame.h / 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: frame.h,
                    y: frame.h,
                }),
                rotation: -entity.direction_angle + PI / 2.0,
                source: Some(texture_sector),
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        attack_animation.tick();
    } else {
        let texture_sector = walk_animation.get_frame();
        let mut frame = walk_animation.get_frame();
        frame.scale(2.0, 2.0);
        draw_texture_ex(
            walk_animation.get_texture(),
            entity.position.x - frame.h / 2.0,
            field.height - (entity.position.y + frame.h / 2.0),
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: frame.h,
                    y: frame.h,
                }),
                rotation: -entity.direction_angle + PI / 2.0,
                source: Some(texture_sector),
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        if entity.speed.length() < SPEED_TO_ANIMATE_WALK {
            return;
        }

        walk_animation.tick();
    }
}

fn draw_steps(field: &Field, global_tick: u32, footstep_tracker: &mut FootstepTracker) {
    footstep_tracker.tick(global_tick); // should it happen here?
    for step in footstep_tracker.footsteps.iter() {
        draw_texture_ex(
            &step.texture,
            step.position.x - step.texture.width() / 2.0,
            field.height - (step.position.y + step.texture.height() / 2.0),
            Color {
                r: (1.0),
                g: (1.0),
                b: (1.0),
                a: (1.0
                    - (global_tick - step.created_at_tick) as f32
                        / footstep_tracker.footstep_lifetime as f32)
                    - 0.5,
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

async fn load_attack_animations() -> AnimationPack {
    let gladiator_walk_texture = Rc::new(
        load_texture("assets/textures/real/gladiator_walking.png")
            .await
            .unwrap(),
    );
    let mut walk_vec = vec![];
    for i in 0..8 {
        walk_vec.push((
            Rect::new(256.0 * (i as f32), 0.0, 256.0, 256.0),
            TICKS_PER_FRAME * 2,
        ));
    }

    let gladiator_attack_texture = Rc::new(
        load_texture("assets/textures/real/gladiator_attack.png")
            .await
            .unwrap(),
    );
    let mut attack_vec = vec![];
    for i in 0..15 {
        attack_vec.push((
            Rect::new(256.0 * (i as f32), 0.0, 256.0, 256.0),
            TICKS_PER_FRAME,
        ));
    }

    let gladiator_step_texture = Rc::new(
        load_texture("assets/textures/real/footstep_left.png")
            .await
            .unwrap(),
    );
    let mut step_vec = vec![];
    for i in 0..8 {
        step_vec.push((
            Rect::new(256.0 * (i as f32), 0.0, 256.0, 256.0),
            TICKS_PER_FRAME * 2,
        ));
    }
    let zombie_texture = Rc::new(load_texture("assets/textures/ai/zombie.png").await.unwrap());
    let mut zombie_vec = vec![];
    for i in 0..8 {
        zombie_vec.push((
            Rect::new(256.0 * (i as f32), 0.0, 256.0, 256.0),
            TICKS_PER_FRAME * 2,
        ));
    }

    return AnimationPack {
        gladiator_walk: ProtoAnimation {
            texture: gladiator_walk_texture,
            frames: walk_vec,
        },
        gladiator_attack: ProtoAnimation {
            texture: gladiator_attack_texture,
            frames: attack_vec,
        },
        gladiator_step_texture: gladiator_step_texture,
        zombie: ProtoAnimation {
            texture: zombie_texture,
            frames: zombie_vec,
        },
    };
}

fn make_zombies() -> Vec<Entity> {
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

    let annimation_pack = load_attack_animations().await;

    let mut engine = Engine::new(
        Entity::make_gladiator(Vec2 { x: 50.0, y: 50.0 }),
        make_zombies(),
        field.clone(),
    );
    let mut attack_animations = HashMap::<u32, Animation>::new();
    attack_animations.insert(
        engine.gladiator.id,
        Animation::new(annimation_pack.gladiator_attack),
    );

    let mut walk_animations = HashMap::<u32, Animation>::new();
    walk_animations.insert(
        engine.gladiator.id,
        Animation::new(annimation_pack.gladiator_walk),
    );
    let mut footstep_trackers = HashMap::<u32, FootstepTracker>::new();
    footstep_trackers.insert(
        engine.gladiator.id,
        FootstepTracker::new(annimation_pack.gladiator_step_texture),
    );
    let mut entities_positions = HashMap::<u32, Vec2>::new();
    entities_positions.insert(engine.gladiator.id, engine.gladiator.position);

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
            engine.gladiator.direction_angle += (0.1) / std::f32::consts::PI;
        }
        if is_key_down(KeyCode::Right) {
            engine.gladiator.direction_angle -= (0.1) / std::f32::consts::PI;
        }
        if is_key_down(KeyCode::Space) {
            engine.attack_by_gladiator()
        }
        if is_key_down(KeyCode::R) {
            engine = Engine::new(
                Entity::make_gladiator(Vec2 { x: 50.0, y: 50.0 }),
                make_zombies(),
                field.clone(),
            );
        }
        if is_key_down(KeyCode::Q) {
            return;
        }

        engine.tick();
        draw_all(
            &engine,
            &mut attack_animations,
            &mut walk_animations,
            &mut footstep_trackers,
        );
        for (id, previous_position) in entities_positions.iter_mut() {
            let footstep_tracker = footstep_trackers.get_mut(id).unwrap(); // TODO: unite
                                                                           // footstep_tracker and previous_position?
            let entity = engine.get_entity(*id);

            match entity {
                Some(e) => {
                    let distance_travelled = (e.position - *previous_position).length();
                    footstep_tracker.add_movement(
                        distance_travelled,
                        e.position,
                        e.direction_angle,
                    );
                    *previous_position = e.position;
                }
                None => todo!(),
            }
        }
        next_frame().await
    }
}
