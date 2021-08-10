use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use sqlparser::ast::Expr;

// https://docs.rs/arrow/5.0.0/arrow/record_batch/struct.RecordBatch.html

/// In-memory storage engine.
///
///
///
/// TODO: Replace this to use Buffer or primitive array
struct MemStorage {
    schema: SchemaRef,
    records: Vec<Vec<RecordBatch>>,
}

impl MemStorage {
    // TODO: What should it return?
    // - Iterator?
    pub fn scan(
        &self, 
        projections: Vec<usize>, 
        filter: Expr, 
        limit: u32) {
    }
}
