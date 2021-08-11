use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::Arc;

use crate::storage::wal::LogRecord;
use crate::storage::redo::RedoRecord;
use crate::storage::slot::Slot;

const BATCH_SIZE: u32 = 32;

struct LogManager {
    writer: LogWriter,
    log_queue: VecDeque<Arc<LogRecord>>,
}

impl LogManager {
    pub fn new() -> Self {
        Self {
            writer: LogWriter::new(),
            log_queue: VecDeque::new(),
        }
    }

    /// Start a thread that process redo record in batches and flush to disk
    pub async fn start(&self) {
        tokio::spawn(async {
            // TODO: Currently this write each log record in sync which isn't 
            // efficient. Perhaps try to batch log records?
            
            // First, copy and clear log_queue
        });
    }

    fn add_to_queue(&mut self, logs: Vec<Arc<LogRecord>>) {
        for log in logs {
            self.log_queue.push_back(log);
        }
    }
}

/// Encapsulates writing LogRecord to disk
struct LogWriter {
    log_filepath: String,
    file: File,
}

impl LogWriter {
    fn new() -> Self {
        // TODO: Don't hardcode log path
        Self::with_path(String::from("wal.log"))
    }

    fn with_path(log_filepath: String) -> Self {
        let file = OpenOptions::new() 
            .append(true)
            .create(true)
            .open(&log_filepath)
            .unwrap();
        Self {
            log_filepath,
            file 
        }
    }

    fn write(&mut self, log_record: &LogRecord) -> std::io::Result<()> {
        // Using unsafe memory cast is hard so for now just use bincode
        /*
        let buf = serialize_log_record(log_record);
        self.file.write_all(&buf)
        */
        let buf = bincode::serialize(log_record).unwrap();
        self.file.write_all(&buf)
    }

    fn flush(&self) {
        self.file.sync_data();
    }
}

/// Encapsulates reading LogRecord from disk
struct LogReader {
    log_filepath: String,
    file: File
}

impl LogReader {
    fn new() -> Self {
        // TODO: Don't hardcode log path
        Self::with_path(String::from("wal.log"))
    }

    fn with_path(log_filepath: String) -> Self {
        let file = File::open(&log_filepath).unwrap();
        Self {
            log_filepath,
            file,
        }
    }

    /// Read the next log record, returning None if EOF is reached
    fn read(&mut self) -> Vec<LogRecord> {
        let mut buf = Vec::<u8>::new();
        self.file.read_to_end(&mut buf);
        let res: Option<LogRecord> = bincode::deserialize(&buf).unwrap();
        // Using unsafe memory cast operation is hard so for now just use 
        // bincode
        /*
        let res = unsafe { std::mem::transmute::<Vec<u8>, Vec<LogRecord>>(buf) };
        */
        vec!(res.unwrap())
    }
}

fn serialize_log_record(log_record: &LogRecord) -> &[u8] {
    let bytes = unsafe { any_as_u8_slice(log_record) };
    bytes
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

#[cfg(test)]
mod tests {
    use arrow::datatypes::{DataType, Field, Schema};

    use super::*;

    #[test]
    fn write_and_read_log() {
        let schema = Schema::new(vec![
            Field::new("a", DataType::Int64, false),
            Field::new("b", DataType::Boolean, false),
        ]);
        let logs = [
            LogRecord::Redo(RedoRecord {
                schema: Arc::new(schema),
                slot: Slot {
                    block_id: 1,
                    offset: 1,
                },
                content: vec![0, 1, 2, 3],
            }),
        ];

        {
            let mut writer = LogWriter::new();
            writer.write(&logs[0]);
            writer.flush();
        }

        let mut reader = LogReader::new();
        let read_logs = reader.read();
        assert_eq!(read_logs, logs);
    }
}

