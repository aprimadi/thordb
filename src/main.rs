use std::io::prelude::Read;
use std::net::TcpListener;

use datafusion::prelude::ExecutionContext;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

mod catalog;
mod storage;
mod transaction;

fn boot() -> std::io::Result<()> {
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:2403")?;

    // Accept incoming connections
    for stream in listener.incoming() {
        let mut stream = stream?;
        tokio::spawn(async move {
            let mut statement = String::from("");
            let mut buf: [u8; 128] = [0; 128];
            loop {
                // Attack plan:
                // - Read stream and append buf until it found a newline `\n`
                // - When it found a newline, check if the current buffer ends with semicolon `;`
                // - When buf ends with semicolon try parsing buf as sql statement, returning error
                //   if statement is invalid
                // - If statement is valid, execute the statement and return the result
                let res = stream.read(&mut buf);
                match res {
                    Ok(nbytes) => {
                        // TODO
                    }
                    Err(e) => {
                        // TODO: want to return error message to client
                        log::error!("");
                    }
                }
            }
        });
    }
    Ok(())
}

fn main() {
    let sql = "SELECT * FROM users";
    let dialect = GenericDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    println!("AST: {:?}", ast[0]);

    let mut ctx = ExecutionContext::new();
    let plan = ctx.create_logical_plan(sql).unwrap();
}

