use macroquad::prelude::*;
use std::{convert::TryInto, u8};

pub trait MySerialize {
    fn serialize(&self, buffer: &mut Vec<u8>);
}

// TODO: implement Rader and Writer
pub trait MyDeserialize: Sized {
    fn deserialize(buffer: &[u8], cursor: &mut usize) -> Result<Self, String>;
}
impl MySerialize for bool {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        if *self == true {
            let bytes: [u8; 1] = [1];
            buffer.extend_from_slice(&bytes);
        } else {
            let bytes: [u8; 1] = [0];
            buffer.extend_from_slice(&bytes);
        }
    }
}
impl MyDeserialize for bool {
    fn deserialize(buffer: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + 4 > buffer.len() {
            return Err("Unexpected End of File reading u32".to_string());
        }
        let value = u8::from_be_bytes(buffer[*cursor..*cursor + 4].try_into().unwrap());
        *cursor += 4;
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err("Unknown state".to_string()),
        }
    }
}
impl MySerialize for usize {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_be_bytes());
    }
}
impl MyDeserialize for usize {
    fn deserialize(buffer: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + size_of::<usize>() > buffer.len() {
            return Err("Unexpected End of File reading usize".to_string());
        }
        let bytes = buffer[*cursor..*cursor + 4].try_into().unwrap();
        *cursor += size_of::<usize>();
        Ok(usize::from_be_bytes(bytes))
    }
}
impl MySerialize for u32 {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.to_be_bytes());
    }
}
impl MySerialize for String {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        let bytes = self.clone().into_bytes(); // TODO: implement without clone
        buffer.extend_from_slice(&bytes.len().to_be_bytes());
        buffer.extend_from_slice(&bytes);
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
impl MyDeserialize for String {
    fn deserialize(buffer: &[u8], cursor: &mut usize) -> Result<Self, String> {
        if *cursor + size_of::<usize>() > buffer.len() {
            return Err("Unexpected End of File reading usize for string length".to_string());
        }
        let string_lenght_in_bytes = usize::from_be_bytes(
            buffer[*cursor..*cursor + size_of::<usize>()]
                .try_into()
                .unwrap(),
        );
        *cursor += size_of::<usize>();

        if *cursor + string_lenght_in_bytes > buffer.len() {
            return Err("Unexpected End of File reading string".to_string());
        }
        let bytes = buffer[*cursor..*cursor + string_lenght_in_bytes]
            .try_into()
            .unwrap();
        *cursor += string_lenght_in_bytes;
        Ok(String::from_utf8(bytes).unwrap())
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
