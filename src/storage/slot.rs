use serde::{Serialize, Deserialize};

/// A slot encapsulates block id and offset position. See memory.rs for how 
/// data are stored in 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Slot {
    pub block_id: u32,
    pub offset: u32,
}

