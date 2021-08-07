
/// A partial row represents a row with only partial columns data.
struct PartialRow {
    size: usize,
    // TODO: Use real bitmap implementation
    null_bitmap: Vec<u8>,
    columns: Vec<ColumnId>,
    values: Vec<String>,
}

impl PartialRow {
    /// Return struct size
    fn size() -> usize {
        0
    }
}
