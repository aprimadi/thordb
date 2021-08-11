mod log_manager;

pub enum LogRecord {
    Redo(RedoRecord),
    Delete(DeleteRecord),
    Commit(CommitRecord),
    Abort(AbortRecord),
}

struct DeleteRecord {
    db_oid: u64,
    table_oid: u64,
    slot: Slot,
}

struct CommitRecord {
    begin_ts: Timestamp,
    commit_ts: Timestamp,
    read_only_txn: bool,
}

/// Abort record is not stored to disk
struct AbortRecord {
}
