use crate::transaction::Timestamp;

type UndoBuffer = Vec<String>;
type RedoBuffer = Vec<String>;

pub struct TransactionContext {
    begin_ts: Timestamp,
    undo_buffer: UndoBuffer,
    redo_buffer: RedoBuffer,
}

impl TransactionContext {
    pub fn new(begin_ts: Timestamp) -> Self {
        Self {
            begin_ts,
            undo_buffer: UndoBuffer::new(),
            redo_buffer: RedoBuffer::new(),
        }
    }

    pub fn begin_ts(&self) -> Timestamp {
        self.begin_ts
    }
}

