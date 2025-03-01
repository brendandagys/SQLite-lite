mod compiler;
mod types;
mod vm;

use compiler::prepare_statement;
use std::io::{self, Write};
use types::{InputBuffer, MetaCommandResult, PrepareResult, Statement, StatementType};
use vm::execute_statement;

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
        io::stdout().flush().unwrap();
    }
}
