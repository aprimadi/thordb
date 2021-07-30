use crate::transaction::TransactionContext;

// TODO: Declare these types
type Txn = String;
type Slot = String;
type ProjectedRow = String;

/// Table is the forward facing API of the storage module. It consists of 
/// Apache Arrow blocks.
struct Table {
    blocks_size: u32,
}

impl Table {
    // TODO: Declare txn type
    // TODO: Replace output_buffer with real buffer 
    fn select(txn: TransactionContext, slot: Slot, output_buffer: &mut Vec<String>) -> bool {
        false
    }

    fn scan(txn: TransactionContext, start_slot: Slot, output_buffer: &mut Vec<String>) {
    }

    // TODO: Implement iterator interface
    
    fn update(txn: Txn, slot: Slot, redo: ProjectedRow) -> bool {
        false
    }

    fn insert(txn: Txn, redo: ProjectedRow) -> Slot {
        "".to_string()
    }

    fn delete(txn: Txn, slot: Slot) -> bool {
        false
    }
}
