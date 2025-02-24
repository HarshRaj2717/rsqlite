use crate::compiler::{self, StatementType};

const UNEXPECTED_ERROR_MESSAGE: &str = "Unexpected Error Encountered";

fn execute_select(table_name: &String) {

}

fn execute_insert(table_name: &String, row_to_insert: &Vec<String>) {

}

pub fn execute(statement: &compiler::Statement) -> () {
    match statement.statement_type() {
        StatementType::Create => todo!(),
        StatementType::Drop => todo!(),
        StatementType::Insert => {
            let table_name: &String = statement.table_name().expect(UNEXPECTED_ERROR_MESSAGE);
            let row_to_insert: &Vec<String> =
                statement.row_to_insert().expect(UNEXPECTED_ERROR_MESSAGE);
            execute_insert(table_name, row_to_insert)
        }
        StatementType::MetaHelp => todo!(),
        StatementType::MetaPrint => {
            let to_print: &String = statement.meta_args().expect("");
            println!("{to_print}");
        }
        StatementType::Select => {
            let table_name = statement.table_name().expect(UNEXPECTED_ERROR_MESSAGE);
            execute_select(table_name)
        }
        StatementType::Undefined => panic!("{UNEXPECTED_ERROR_MESSAGE}"),
    }
}
