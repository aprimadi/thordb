use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::storage::serde::{
    Serialize, Deserialize, 
    MSG_SERIALIZE_ERROR, MSG_DESERIALIZE_ERROR
};

/// A slot encapsulates block id and offset position. See memory.rs for how 
/// data are stored in 
#[derive(Debug, PartialEq)]
pub struct Slot {
    pub block_id: u32,
    pub offset: u32,
}

impl Slot {
    pub fn empty() -> Self {
        Self {
            block_id: 0,
            offset: 0,
        }
    }
}

impl Serialize for Slot {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.write_u32::<BigEndian>(self.block_id).expect(MSG_SERIALIZE_ERROR);
        res.write_u32::<BigEndian>(self.offset).expect(MSG_SERIALIZE_ERROR);
        res
    }
}

impl Deserialize for Slot {
    fn deserialize(&mut self, rdr: &mut Cursor<Vec<u8>>) {
        self.block_id = rdr.read_u32::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        self.offset = rdr.read_u32::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
    }
}

