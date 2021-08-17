use std::io::Cursor;

pub const MSG_SERIALIZE_ERROR: &str = "serialize log record error";
pub const MSG_DESERIALIZE_ERROR: &str = "deserialize log record error";

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserialize {
    fn deserialize(&mut self, rdr: Cursor<Vec<u8>>);
}

