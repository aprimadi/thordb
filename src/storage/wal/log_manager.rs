use std::fs::File;

use crate::storage::wal::LogRecord;

const BATCH_SIZE = 32;

struct LogManager {
    writer: LogWriter,
    log_queue: Queue<Arc<RedoRecord>>,
}

impl LogManager {
    pub fn new() -> Self {
        Self {
            log_queue: Queue::new(),
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

    fn add_to_queue(&self, logs: Vec<Arc<LogRecord>>) {
        for log in logs {
            self.log_queue.add(log);
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
        with_path(String::from("wal.log"))
    }

    fn with_path(log_filepath: String) -> Self {
        let file = File::with_options()
            .append(true)
            .create(true)
            .open(log_filepath)
            .unwrap();
        Self {
            log_filepath,
            file 
        }
    }

    fn write(&self, log_record: &LogRecord) -> Result<usize> {
        let buf = self.serialize(log_record);
        self.file.write_all(&buf)
    }

    fn serialize(&self, log_record: &LogRecord) -> &[u8] {
        let bytes = unsafe { std::mem::transmute::<&LogRecord, &[u8]>(log_record) }
        bytes
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
        with_path(String::from("wal.log"))
    }

    fn with_path(log_filepath: String) -> Self {
        let file = File::open(log_filepath).unwrap();
        Self {
            log_filepath,
            file,
        }
    }

    /// Read the next log record, returning None if EOF is reached
    fn read(&self) -> Option<LogRecord> {
        // TODO
        None
    }
}

