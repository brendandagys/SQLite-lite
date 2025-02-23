use std::io::{self, Write};

struct InputBuffer {
    buffer: String,
}

impl InputBuffer {
    fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    fn read_input(&mut self) {
        self.buffer.clear();
        io::stdin()
            .read_line(&mut self.buffer)
            .expect("Failed to read line");
        self.buffer = self.buffer.trim_end().to_string(); // Remove trailing `\n`
    }
}

fn print_prompt() {
    print!("sqlite-lite> ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        input_buffer.read_input();

        if input_buffer.buffer == ".exit" {
            std::process::exit(0);
        } else {
            println!("Unrecognized command '{}'.", input_buffer.buffer);
            io::stdout().flush().unwrap();
        }
    }
}
