use arrow::datatypes::SchemaRef;
use serde::{Serialize, Deserialize};

use crate::storage::slot::Slot;

struct RedoBuffer {
    record: Vec<RedoRecord>,
}

impl RedoBuffer {
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RedoRecord {
    pub schema: SchemaRef,
    pub slot: Slot,
    pub content: Vec<u8>,
}

