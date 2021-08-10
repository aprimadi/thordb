use arrow::datatypes::SchemaRef;

use crate::storage::slot::Slot;

struct RedoBuffer {
    record: Vec<RedoRecord>,
}

impl RedoBuffer {
}

struct RedoRecord {
    schema: SchemaRef,
    slot: Slot,
    content: Vec<u8>,
}
