type Slot = String;

pub enum DeltaRecordType {
    Update,
    Insert,
    Delete,
}

/// Record buffer segment stores vector of UndoRecordRef
type UndoRecordRef = Arc<UndoRecord>;

/// UndoRecord is chained version linked list from newest-to-oldest.
/// The newest record is stored in the table.
struct UndoRecord {
    record_type: DeltaRecordType,
    next: Box<UndoRecord>,
    timestamp: Timestamp,
    table: Table,
    slot: Slot,
    content: PartialRow,
}

impl UndoRecord {
    /// Return struct size
    fn size() -> usize {
        0
    }
}
