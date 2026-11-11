use std::rc::Rc;

use macroquad::prelude::*;

use crate::serialization::{MyDeserialize, MySerialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Animation {
    path_to_texture: String,
    texture: Rc<Texture2D>,
    frames: Vec<(Rect, u32)>,
    current_frame: usize,
    current_tick: u32,
}

impl MySerialize for Animation {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        self.path_to_texture.serialize(buffer);
        self.current_tick.serialize(buffer);
        self.current_frame.serialize(buffer);
    }
}

impl MyDeserialize for Animation {
    fn deserialize(
        buffer: &[u8],
        cursor: &mut usize,
        texture_pack_map: HashMap<String, (Rc<Texture2D>, Vec<(Rect, u32)>)>,
    ) -> Result<Self, String> {
        let path_to_texture = String::deserialize(buffer, cursor)?;
        let (texture, frames) = texture_pack_map[&path_to_texture].clone();
        Ok(Animation {
            path_to_texture,
            texture: texture,
            frames: frames,
            current_frame: usize::deserialize(buffer, cursor)?,
            current_tick: u32::deserialize(buffer, cursor)?,
        })
    }
}

impl Animation {
    pub fn new(
        path_to_texture: String,
        texture_pack_map: HashMap<String, (Rc<Texture2D>, Vec<(Rect, u32)>)>,
    ) -> Self {
        let (texture, frames) = texture_pack_map[&path_to_texture].clone();
        return Self {
            path_to_texture: path_to_texture,
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
