use crate::types::{InputBuffer, PrepareResult, Statement, StatementType};

pub fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
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
