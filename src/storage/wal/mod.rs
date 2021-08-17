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

#[derive(Debug, PartialEq)]
pub enum LogRecord {
    Redo(RedoRecord),
    Delete(DeleteRecord),
    Commit(CommitRecord),
    Abort(AbortRecord),
}

#[derive(Debug, PartialEq)]
pub struct DeleteRecord {
    db_oid: u64,
    table_oid: u64,
    slot: Slot,
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
    fn deserialize(&mut self, rdr: Cursor<Vec<u8>>) {
        self.db_oid = rdr.read_u64::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        self.table_oid = rdr.read_u64::<BigEndian>().expect(MSG_DESERIALIZE_ERROR);
        let pos = rdr.position();
        // TODO
    }
}

#[derive(Debug, PartialEq)]
pub struct CommitRecord {
    begin_ts: Timestamp,
    commit_ts: Timestamp,
    read_only_txn: bool,
}

impl Serialize for CommitRecord {
    fn serialize(&self) -> Vec<u8> {
        let res = Vec::new();
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
    fn deserialize(&mut self, rdr: Cursor<Vec<u8>>) {
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

impl Serialize for AbortRecord {
    fn serialize(&self) -> Vec<u8> {
        let res = Vec::new();
        res
    }
}

impl Deserialize for AbortRecord {
    fn deserialize(&mut self, bytes: Cursor<Vec<u8>>) {
        // Nothing since nothing is stored
    }
}
