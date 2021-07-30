type UndoBuffer = Vec<String>;
type RedoBuffer = Vec<String>;

pub struct TransactionContext {
    undo_buffer: UndoBuffer,
    redo_buffer: RedoBuffer,
}

impl TransactionContext {
}
