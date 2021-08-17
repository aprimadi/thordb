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
    /// TODO: Write doc on why we need to use Arc for LogRecord?
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
/// 
/// Log format
///
///  --------------------------------------------------------
/// | size_1 | log_1 | size_2 | log_2 | ... | size_N | log_N |
///  --------------------------------------------------------
///
/// Explanation:
/// The write always write the size of the log struct before the log struct 
/// itself. This way the reader know the boundary of the next log entry to 
/// read.
///
/// There might be a way to encode the size and the rest of the struct 
/// information as part of the struct but this requires some pointer magic 
/// that is best to avoid for now.
///
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
        let struct_buf = bincode::serialize(log_record).unwrap();
        let size_buf = bincode::serialize(&struct_buf.len()).unwrap();
        let res: LogRecord = bincode::deserialize(&struct_buf[..]).unwrap();
        self.file.write_all(&size_buf)?;
        self.file.write_all(&struct_buf)
    }

    fn flush(&self) {
        self.file.sync_data();
    }
}

const USIZE_LEN: usize = std::mem::size_of::<usize>();

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
    fn read(&mut self) -> LogRecord {
        // Read size
        let mut size_buf: [u8; USIZE_LEN] = [0; USIZE_LEN];
        self.file.read_exact(&mut size_buf);
        let size: usize = bincode::deserialize(&size_buf).unwrap();

        let mut struct_buf = vec![0u8; size];
        self.file.read_exact(&mut struct_buf);
        let res: LogRecord = bincode::deserialize(&struct_buf[..]).unwrap();
        // Using unsafe memory cast operation is hard so for now just use 
        // bincode
        /*
        let res = unsafe { std::mem::transmute::<Vec<u8>, Vec<LogRecord>>(buf) };
        */
        res
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
        let log1 = reader.read();
        assert_eq!(log1, logs[0]);
    }
}

