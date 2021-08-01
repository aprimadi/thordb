use std::collections::HashSet;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::transaction::TransactionContext;
use crate::transaction::Timestamp;

/// TimestampManager keeps track of the timestamp used for MVCC and tracks 
/// timestamp of running transactions.
///
/// The tick is reset every time the db is restarted.
/// TODO: Why is this okay?
struct TimestampManager {
    timestamp: AtomicU64,
    // TODO: Use more efficient data structure
    active_txn_timestamps: Mutex<HashSet<Timestamp>>,
}

impl TimestampManager {
    fn new() -> Self {
        Self {
            timestamp: AtomicU64::new(0),
            active_txn_timestamps: Mutex::new(HashSet::new()),
        }
    }

    /// Get unique timestamp and advances by one tick
    fn generate(&self) -> Timestamp {
        self.timestamp.fetch_add(1, Ordering::Relaxed)
    }

    /// Return unique timestamp without advancing
    fn current(&self) -> Timestamp {
        self.timestamp.load(Ordering::Relaxed)
    }

    /// Return the oldest transaction alive begin timestamp
    fn oldest_txn_begin_ts() -> Option<Timestamp> {
        None
    }

    fn add_txn(&mut self, ts: u64) {
        let mut txns = self.active_txn_timestamps.lock().unwrap();
        txns.insert(ts);
    }

    /// Remove the given timestamp from running transactions
    fn remove_txn(&mut self, ts: u64) {
        let mut txns = self.active_txn_timestamps.lock().unwrap();
        txns.remove(&ts);
    }
}

struct TransactionManager {
    timestamp_mgr: TimestampManager,
}

impl TransactionManager {
    fn begin(&self) -> TransactionContext {
        let begin_ts = self.timestamp_mgr.generate();
        let ctx = TransactionContext::new(begin_ts);
        ctx
    }

    fn commit(&mut self, txn: TransactionContext) -> Timestamp {
        // TODO
        self.timestamp_mgr.remove_txn(txn.begin_ts());
        0
    }

    fn abort(&mut self, txn: TransactionContext) -> Timestamp {
        // TODO
        self.timestamp_mgr.remove_txn(txn.begin_ts());
        0
    }
}

