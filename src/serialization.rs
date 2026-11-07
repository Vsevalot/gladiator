use crate::entity::{Entity, Weapon};
use macroquad::prelude::*;
use std::convert::TryInto;

pub trait MySerialize {
    fn serialize(&self, buffer: &mut Vec<u8>);
}

pub trait MyDeserialize: Sized {
    fn deserialize(buffer: &[u8], cursor: &mut usize) -> Result<Self, String>;
}

impl MySerialize for u32 {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_be_bytes());
    }
}
impl MySerialize for f32 {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_be_bytes());
    }
}
impl MySerialize for Vec2 {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        self.x.serialize(buffer);
        self.y.serialize(buffer);
    }
}
impl MyDeserialize for u32 {
    fn deserialize(buffer: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + 4 > buffer.len() {
            return Err("Unexpected End of File reading u32".to_string());
        }
        let bytes = buffer[*cursor..*cursor + 4].try_into().unwrap();
        *cursor += 4;
        Ok(u32::from_be_bytes(bytes))
    }
}
impl MyDeserialize for f32 {
    fn deserialize(buffer: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + 4 > buffer.len() {
            return Err("Unexpected End of File reading f32".to_string());
        }
        let bytes = buffer[*cursor..*cursor + 4].try_into().unwrap();
        *cursor += 4;
        Ok(f32::from_be_bytes(bytes))
    }
}
impl MyDeserialize for Vec2 {
    fn deserialize(buffer: &[u8], cursor: &mut usize) -> Result<Self, String> {
        Ok(Vec2 {
            x: f32::deserialize(buffer, cursor)?,
            y: f32::deserialize(buffer, cursor)?,
        })
    }
}
