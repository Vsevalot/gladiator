use std::rc::Rc;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct ProtoAnimation {
    pub texture: Rc<Texture2D>,
    pub frames: Vec<(Rect, u32)>,
}

#[derive(Debug, Clone)]
pub struct Animation {
    proto_animation: ProtoAnimation,
    current_frame: usize,
    current_tick: u32,
}

impl Animation {
    pub fn new(proto_animation: ProtoAnimation) -> Self {
        return Self {
            proto_animation: proto_animation,
            current_frame: 0,
            current_tick: 0,
        };
    }

    pub fn tick(&mut self) {
        let (_, ticks_per_frame) = self.proto_animation.frames[self.current_frame];

        self.current_tick += 1;

        if self.current_tick >= ticks_per_frame {
            self.current_tick = 0;
            self.current_frame += 1;
            if self.current_frame >= self.proto_animation.frames.len() {
                self.current_frame = 0;
            }
        }
    }

    pub fn get_texture(&self) -> &Texture2D {
        return &self.proto_animation.texture;
    }
    pub fn get_frame(&self) -> Rect {
        let (frame_rect, _) = self.proto_animation.frames[self.current_frame];
        return frame_rect;
    }
}
