use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

mod catalog;
mod storage;
mod transaction;

fn main() {
    let sql = "SELECT * FROM users";
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    println!("AST: {:?}", ast[0]);
}

