mod types;

use std::io::{self, Write};
use types::{InputBuffer, MetaCommandResult, PrepareResult, Statement, StatementType};

fn print_prompt() {
    print!("sqlite-lite> ");
    io::stdout().flush().unwrap();
}

fn do_meta_command(input_buffer: &InputBuffer) -> MetaCommandResult {
    if input_buffer.buffer == ".exit" {
        std::process::exit(0);
    } else {
        return MetaCommandResult::UnrecognizedCommand;
    }
}

fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    if input_buffer.buffer.starts_with("insert") {
        statement.type_ = StatementType::Insert;
        return PrepareResult::Success;
    }

    if input_buffer.buffer.starts_with("select") {
        statement.type_ = StatementType::Select;
        return PrepareResult::Success;
    }

    PrepareResult::UnrecognizedStatement
}

fn execute_statement(statement: Statement) -> () {
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

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        input_buffer.read_input();

        if (&input_buffer).buffer.starts_with(".") {
            match do_meta_command(&input_buffer) {
                MetaCommandResult::Success => continue,
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized command '{}'.", input_buffer.buffer);
                    io::stdout().flush().unwrap();
                    continue;
                }
            }
        }

        let mut statement = Statement {
            type_: StatementType::None,
        };

        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::Success => {}
            PrepareResult::UnrecognizedStatement => {
                println!(
                    "Unrecognized keyword at start of '{}'.",
                    input_buffer.buffer
                );
                io::stdout().flush().unwrap();
                continue;
            }
        }

        execute_statement(statement);
        println!("Executed.");
    }
}
