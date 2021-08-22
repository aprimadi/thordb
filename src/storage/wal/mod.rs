use std::convert::{TryFrom, TryInto};
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

mod log_manager;

use crate::storage::redo::RedoRecord;
use crate::storage::slot::Slot;
use crate::storage::serde::{
    Serialize, Deserialize, 
    MSG_SERIALIZE_ERROR, MSG_DESERIALIZE_ERROR,
};
use crate::transaction::Timestamp;

enum LogRecordType {
    Redo = 1,
    Delete,
    Commit,
    Abort,
}

impl TryFrom<u8> for LogRecordType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == LogRecordType::Redo as u8     => Ok(LogRecordType::Redo),
            x if x == LogRecordType::Delete as u8   => Ok(LogRecordType::Delete),
            x if x == LogRecordType::Commit as u8   => Ok(LogRecordType::Commit),
            x if x == LogRecordType::Abort as u8    => Ok(LogRecordType::Abort),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LogRecord {
    Redo(RedoRecord),
    Delete(DeleteRecord),
    Commit(CommitRecord),
    Abort(AbortRecord),
}

impl LogRecord {
    fn deserialize(bytes: Vec<u8>) -> LogRecord {
        let mut rdr = Cursor::new(bytes);
        let lr_type: u8 = rdr.read_u8().expect(MSG_DESERIALIZE_ERROR);
        let lr_type: LogRecordType = lr_type.try_into().unwrap();
        // TODO: Implement deserialize
        match lr_type {
            LogRecordType::Redo => {
                let mut redo = RedoRecord::empty();
                redo.deserialize(&mut rdr);
                LogRecord::Redo(redo)
            },
            LogRecordType::Delete => {
                let mut delete = DeleteRecord::empty();
                delete.deserialize(&mut rdr);
                LogRecord::Delete(delete)
            },
            LogRecordType::Commit => {
                let mut commit = CommitRecord::empty();
                commit.deserialize(&mut rdr);
                LogRecord::Commit(commit)
            },
            LogRecordType::Abort => {
                let mut abort = AbortRecord::empty();
                abort.deserialize(&mut rdr);
                LogRecord::Abort(abort)
            },
        }
    }
}

impl Serialize for LogRecord {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        match &self {
            LogRecord::Redo(rec) => {
                res.write_u8(LogRecordType::Redo as u8).expect(MSG_SERIALIZE_ERROR);
                let mut bytes = rec.serialize();
                res.append(&mut bytes);
            },
            LogRecord::Delete(rec) => {
                res.write_u8(LogRecordType::Delete as u8).expect(MSG_SERIALIZE_ERROR);
                let mut bytes = rec.serialize();
                res.append(&mut bytes);
            },
            LogRecord::Commit(rec) => {
                res.write_u8(LogRecordType::Commit as u8).expect(MSG_SERIALIZE_ERROR);
                let mut bytes = rec.serialize();
                res.append(&mut bytes);
            },
            LogRecord::Abort(rec) => {
                res.write_u8(LogRecordType::Abort as u8).expect(MSG_SERIALIZE_ERROR);
                let mut bytes = rec.serialize();
                res.append(&mut bytes);
            }
        }
        res
    }
}

#[derive(Debug, PartialEq)]
pub struct DeleteRecord {
    db_oid: u64,
    table_oid: u64,
    slot: Slot,
}

impl DeleteRecord {
    fn empty() -> Self {
        Self {
            db_oid: 0,
            table_oid: 0,
            slot: Slot::empty(),
        }
    }
}

impl Serialize for DeleteRecord {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.write_u64::<BigEndian>(self.db_oid).expect(MSG_SERIALIZE_ERROR);
        res.write_u64::<BigEndian>(self.table_oid).expect(MSG_SERIALIZE_ERROR);
        let mut slot_bytes = self.slot.serialize();
        res.append(&mut slot_bytes);
        res
    }
}

impl Deserialize for DeleteRecord {
    fn deserialize(&mut self, rdr: &mut Cursor<Vec<u8>>) {
        self.db_oid = rdr.read_u64::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        self.table_oid = rdr.read_u64::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        self.slot.deserialize(rdr);
    }
}

#[derive(Debug, PartialEq)]
pub struct CommitRecord {
    begin_ts: Timestamp,
    commit_ts: Timestamp,
    read_only_txn: bool,
}

impl CommitRecord {
    pub fn empty() -> Self {
        Self {
            begin_ts: 0,
            commit_ts: 0,
            read_only_txn: false,
        }
    }
}

impl Serialize for CommitRecord {
    fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.write_u64::<BigEndian>(self.begin_ts).expect(MSG_SERIALIZE_ERROR);
        res.write_u64::<BigEndian>(self.commit_ts).expect(MSG_SERIALIZE_ERROR);
        if self.read_only_txn {
            res.write_u8(1).expect(MSG_SERIALIZE_ERROR);
        } else {
            res.write_u8(0).expect(MSG_SERIALIZE_ERROR);
        }
        res
    }
}

impl Deserialize for CommitRecord {
    fn deserialize(&mut self, rdr: &mut Cursor<Vec<u8>>) {
        self.begin_ts = rdr.read_u64::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        self.commit_ts = rdr.read_u64::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        let txn_bit = rdr.read_u8().expect(MSG_DESERIALIZE_ERROR);
        self.read_only_txn = txn_bit != 0;
    }
}

/// Abort record is not stored to disk
#[derive(Debug, PartialEq)]
pub struct AbortRecord {
}

impl AbortRecord {
    pub fn empty() -> Self {
        Self {}
    }
}

impl Serialize for AbortRecord {
    fn serialize(&self) -> Vec<u8> {
        let res = Vec::new();
        res
    }
}

impl Deserialize for AbortRecord {
    fn deserialize(&mut self, bytes: &mut Cursor<Vec<u8>>) {
        // Nothing since nothing is stored
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_record_serialize() {
        let commit = CommitRecord { begin_ts: 1, commit_ts: 2, read_only_txn: true };
        let log_record = LogRecord::Commit(commit);
        let res = log_record.serialize();
        let expected: Vec<u8> = vec!(3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 1);
        assert_eq!(res, expected);
    }
}

