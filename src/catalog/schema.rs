use std::collections::HashMap;

// See here for arrow supported data type.
// https://docs.rs/arrow/5.0.0/arrow/datatypes/enum.DataType.html
//
// Also want to read Arrow physical format:
// https://arrow.apache.org/docs/format/Columnar.html#physical-memory-layout

/// A schema contains a list of columns present in the schema.
struct Schema {
    columns: Vec<Column>,
    col_oid_to_offset: HashMap<u32, usize>,
}

impl Schema {
    fn new(columns: Vec<Column>) -> Self {
        let mut col_oid_to_offset = HashMap::new();
        for (offset, col) in columns.iter().enumerate() {
            col_oid_to_offset.insert(col.oid, offset);
        }
        Self {
            columns,
            col_oid_to_offset,
        }
    }
}

/// Valid SQL types
// TODO: How to represent TEXT?
enum Type {
    Invalid,
    Boolean,
    Tinyint,
    Smallint,
    Integer,
    Bigint,
    Real,
    Decimal,
    Timestamp,
    Date,
    Varchar,
    Varbinary,
    ParameterOffset,
    Variadic,
    VarArray,
}

/// Column type definitions.
/// 
/// oid is a unique identifier for this column.
struct Column {
    name:           String,
    datatype:       String, // TODO: use enum
    attr_length:    usize,
    nullable:       bool,
    default_value:  String, // TODO: should be an expression
    oid:            u32,
}

