mod compiler;
mod virtual_machine;

struct InputBuffer {
    buffer: String,
}

impl InputBuffer {
    fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    fn clear(&mut self) {
        self.buffer = String::new();
    }

    fn print_prompt(&self, multi_line: bool) {
        if multi_line {
            print!("       > ");
        } else {
            print!("rsqlite> ");
        }
        match std::io::Write::flush(&mut std::io::stdout()) {
            Ok(_) => {}
            Err(error) => {
                println!("\nFailed to flush output : {error}");
                std::process::exit(1);
            }
        }
    }

    fn read_multiline(&self, mut multi_line_input: String) -> String {
        loop {
            if let Some(last_char) = multi_line_input.chars().last() {
                if last_char == ';' {
                    break;
                }
            }
            self.print_prompt(true);
            let mut cur_input = String::new();
            match std::io::stdin().read_line(&mut cur_input) {
                Ok(_) => {
                    cur_input = "\n".to_string() + cur_input.trim();
                    multi_line_input.push_str(&cur_input);
                }
                Err(error) => {
                    println!("\nFailed to read input : {error}");
                    std::process::exit(1);
                }
            }
        }
        multi_line_input
    }

    fn read(&mut self) -> String {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
                if let Some(first_char) = input.chars().next() {
                    if first_char != '.' {
                        // SQL queries/statements end with a ';' and can be multi-line
                        // while SQLite commands aka meta-commands start with a '.' and can not be multi-line
                        // here I just get the multi-line input from user for queries
                        input = self.read_multiline(input);
                    }
                    self.buffer = input;
                }
            }
            Err(error) => {
                println!("\nFailed to read input : {error}");
                std::process::exit(1);
            }
        }
        self.buffer.clone()
    }
}

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        input_buffer.clear();
        input_buffer.print_prompt(false);

        let cur_buffer = input_buffer.read();
        if cur_buffer == "" {
            continue;
        }

        let (command_result, statement) = compiler::compile(&cur_buffer);
        if matches!(command_result, compiler::CommandResult::CommandUnrecognized) {
            println!("\n~~~\nUnrecognized command `{cur_buffer}` .\n~~~\n");
            continue;
        }

        virtual_machine::execute(&statement);
    }
}
