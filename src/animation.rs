use std::rc::Rc;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Animation {
    texture: Rc<Texture2D>,
    frames: Vec<(Rect, u32)>,
    current_frame: usize,
    current_tick: u32,
}

impl Animation {
    pub fn new(texture: Rc<Texture2D>, frames: Vec<(Rect, u32)>) -> Self {
        return Self {
            texture: texture,
            frames: frames,
            current_frame: 0,
            current_tick: 0,
        };
    }

    pub fn tick(&mut self) {
        let (_, ticks_per_frame) = self.frames[self.current_frame];

        self.current_tick += 1;

        if self.current_tick >= ticks_per_frame {
            self.current_tick = 0;
            self.current_frame += 1;
            if self.current_frame >= self.frames.len() {
                self.current_frame = 0;
            }
        }
    }

    pub fn get_texture(&self) -> &Texture2D {
        return &self.texture;
    }
    pub fn get_frame(&self) -> Rect {
        let (frame_rect, _) = self.frames[self.current_frame];
        return frame_rect;
    }
}
