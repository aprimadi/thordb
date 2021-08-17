use arrow::datatypes::SchemaRef;

use crate::storage::slot::Slot;

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

