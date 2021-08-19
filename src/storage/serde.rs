use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub const MSG_SERIALIZE_ERROR: &str = "serialize log record error";
pub const MSG_DESERIALIZE_ERROR: &str = "deserialize log record error";

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize {
    fn deserialize(&mut self, rdr: &mut Cursor<Vec<u8>>);
}

pub fn serialize_u8_vec(res: &mut Vec<u8>, data: &Vec<u8>) {
    let content_size = data.len();
    serialize_usize(res, content_size);
    for d in data {
        res.write_u8(*d);
    }
}

pub fn deserialize_u8_vec(rdr: &mut Cursor<Vec<u8>>) -> Vec<u8> {
    let mut res = Vec::new();
    let size = deserialize_usize(rdr);
    for _ in 0..size {
        let b = rdr.read_u8().expect(MSG_DESERIALIZE_ERROR);
        res.push(b);
    }
    res
}

pub fn serialize_usize(res: &mut Vec<u8>, size: usize) {
    if cfg!(target_pointer_width = "64") {
        let size = size as u64;
        res.write_u64::<BigEndian>(size).expect(MSG_SERIALIZE_ERROR);
    } else if cfg!(target_pointer_width = "32") {
        let size = size as u32;
        res.write_u32::<BigEndian>(size).expect(MSG_SERIALIZE_ERROR);
    } else {
        let size = size as u16;
        res.write_u16::<BigEndian>(size).expect(MSG_SERIALIZE_ERROR);
    }
}

pub fn deserialize_usize(rdr: &mut Cursor<Vec<u8>>) -> usize {
    if cfg!(target_pointer_width = "64") {
        let size = rdr.read_u64::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        let size = size as usize;
        size
    } else if cfg!(target_pointer_width = "32") {
        let size = rdr.read_u32::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        let size = size as usize;
        size
    } else {
        let size = rdr.read_u16::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        let size = size as usize;
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_u8_vec() {
        let v: Vec<u8> = vec!(1, 2, 3);
        // Serialize
        let mut res = Vec::new();
        serialize_u8_vec(&mut res, &v);
        // Deserialize
        let v_out = deserialize_u8_vec(&mut Cursor::new(res));
        assert_eq!(v_out, v);
    }
}

