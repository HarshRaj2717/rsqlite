pub enum CommandResult {
    CommandSuccess,
    CommandUnrecognized,
}

pub enum StatementType {
    Insert,
    Undefined,
    Meta,
    Select,
}

/// Internal representation of input for forwarding to virtual machine
pub struct Statement {
    statement_type: StatementType,
}

impl Statement {
    fn new(statement_type: StatementType) -> Self {
        Self { statement_type }
    }
}

fn get_first_word(input: &String) -> String {
    let mut first_word = String::new();
    for c in input.chars() {
        if c == ';' || c == '\n' {
            break;
        }
        if c == ' ' || c == '\t' {
            if first_word.is_empty() {
                continue;
            }
            break;
        }
        first_word.push(c);
    }
    first_word
}

fn compile_meta(input: &String) -> (CommandResult, Statement) {
    let first_word = get_first_word(input).to_lowercase();
    match first_word.as_str() {
        ".exit" => std::process::exit(0),
        _ => (
            CommandResult::CommandUnrecognized,
            Statement::new(StatementType::Undefined),
        ),
    }
}

fn compile_statement(input: &String) -> (CommandResult, Statement) {
    let first_word = get_first_word(input).to_lowercase();
    match first_word.as_str() {
        "insert" => (
            CommandResult::CommandSuccess,
            Statement::new(StatementType::Insert),
        ),
        "select" => (
            CommandResult::CommandSuccess,
            Statement::new(StatementType::Select),
        ),
        _ => (
            CommandResult::CommandUnrecognized,
            Statement::new(StatementType::Undefined),
        ),
    }
}

/// Parse an input string and return its internal representation
pub fn compile(input: &String) -> (CommandResult, Statement) {
    if let Some(first_char) = input.chars().next() {
        match first_char {
            '.' => compile_meta(input),
            _ => compile_statement(input),
        }
    } else {
        (
            CommandResult::CommandUnrecognized,
            Statement::new(StatementType::Undefined),
        )
    }
}
