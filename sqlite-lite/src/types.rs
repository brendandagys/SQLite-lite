use std::io;

pub struct InputBuffer {
    pub buffer: String,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn read_input(&mut self) {
        self.buffer.clear();
        io::stdin()
            .read_line(&mut self.buffer)
            .expect("Failed to read line");
        self.buffer = self.buffer.trim_end().to_string(); // Remove trailing `\n`
    }
}

pub enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

pub enum StatementType {
    None,
    Insert,
    Select,
}

pub struct Statement {
    pub type_: StatementType,
}

pub enum PrepareResult {
    Success,
    UnrecognizedStatement,
}
