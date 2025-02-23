use crate::types::{Statement, StatementType};

pub fn execute_statement(statement: Statement) -> () {
    match statement.type_ {
        StatementType::Insert => {
            println!("This is where we would do an insert...");
        }
        StatementType::Select => {
            println!("This is where we would do a select...");
        }
        _ => {}
    }
}
