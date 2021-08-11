use serde::{Serialize, Deserialize};

mod log_manager;

use crate::storage::redo::RedoRecord;
use crate::storage::slot::Slot;
use crate::transaction::Timestamp;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LogRecord {
    Redo(RedoRecord),
    Delete(DeleteRecord),
    Commit(CommitRecord),
    Abort(AbortRecord),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteRecord {
    db_oid: u64,
    table_oid: u64,
    slot: Slot,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitRecord {
    begin_ts: Timestamp,
    commit_ts: Timestamp,
    read_only_txn: bool,
}

/// Abort record is not stored to disk
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AbortRecord {
}
