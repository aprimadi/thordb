/// TimestampManager keeps track of the timestamp used for MVCC.
///
/// The tick is reset every time the db is restarted.
/// TODO: Why is this okay?
struct TimestampManager {
    timestamp: AtomicU64,
}

impl TimestampManager {
    /// Get unique timestamp and advances by one tick
    fn get() -> u64 {
    }

    /// Return unique timestamp without advancing
    fn current() -> u64 {
    }

    /// Return the oldest transaction alive begin timestamp
    fn oldest_txn_begin_ts() -> u64 {
    }

    /// TODO: Why is this needed?
    fn cached_oldest_txn_begin_ts() -> u64 {
    }
}

struct TransactionManager {
}

impl TransactionManager {
    fn begin() -> TransactionContext {
    }

    fn commit(txn: TransactionContext) -> Timestamp {
    }

    fn abort(txn: TransactionContext) -> Timestamp {
    }
}

