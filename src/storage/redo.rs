use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::storage::slot::Slot;
use crate::storage::serde::{
    Serialize, Deserialize,
    serialize_u8_vec, deserialize_u8_vec,
    MSG_SERIALIZE_ERROR, MSG_DESERIALIZE_ERROR,
};

struct RedoBuffer {
    record: Vec<RedoRecord>,
}

impl RedoBuffer {
}

#[derive(Debug, PartialEq)]
pub struct RedoRecord {
    // The combination db_oid and table_oid is used to get SchemaRef
    pub db_oid: u64,
    pub table_oid: u64,
    pub slot: Slot,
    pub content: Vec<u8>,
}

impl RedoRecord {
    pub fn empty() -> Self {
        Self {
            db_oid: 0,
            table_oid: 0,
            slot: Slot::empty(),
            content: Vec::new(),
        }
    }
}

impl Serialize for RedoRecord {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.write_u64::<BigEndian>(self.db_oid).expect(MSG_SERIALIZE_ERROR);
        res.write_u64::<BigEndian>(self.table_oid).expect(MSG_SERIALIZE_ERROR);
        let mut slot_bytes = self.slot.serialize();
        res.append(&mut slot_bytes);
        serialize_u8_vec(&mut res, &self.content);
        res
    }
}

impl Deserialize for RedoRecord {
    fn deserialize(&mut self, rdr: &mut Cursor<Vec<u8>>) {
        self.db_oid = rdr.read_u64::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        self.table_oid = rdr.read_u64::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        self.slot.deserialize(rdr);
        self.content = deserialize_u8_vec(rdr);
    }
}

