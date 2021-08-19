use std::io::Cursor;

use arrow::datatypes::SchemaRef;

use crate::storage::slot::Slot;
use crate::storage::serde::{
    Serialize, Deserialize,
    MSG_SERIALIZE_ERROR, MSG_DESERIALIZE_ERROR,
};

struct RedoBuffer {
    record: Vec<RedoRecord>,
}

impl RedoBuffer {
}

#[derive(Debug, PartialEq)]
pub struct RedoRecord {
    pub schema: SchemaRef,
    pub slot: Slot,
    pub content: Vec<u8>,
}

impl Serialize for RedoRecord {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        // TODO: Write schema ref
        let mut slot_bytes = self.slot.serialize();
        res.append(&mut slot_bytes);
        // The size of the content of redo record typically fit into an u32 
        // type, so we'll cast usize into u32 here.
        let content_size = self.content.len() as u32;
        res
    }
}

impl Deserialize for RedoRecord {
    fn deserialize(&mut self, rdr: &mut Cursor<Vec<u8>>) {
    }
}
