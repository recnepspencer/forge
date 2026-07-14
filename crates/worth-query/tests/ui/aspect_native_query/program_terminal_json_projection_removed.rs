use worth_query::facade::policy::{WorthQueryOperationInput, WorthQueryOperationOutput, WorthQueryProgramValue};

fn main() {
    let value = WorthQueryProgramValue::string("native");
    let _ = value.terminal_json_projection();

    let input = WorthQueryOperationInput::new("input", WorthQueryProgramValue::string("native"));
    let _ = input.terminal_json_value_projection();

    let output = WorthQueryOperationOutput::new("output", WorthQueryProgramValue::string("native"));
    let _ = output.terminal_json_value_projection();
}
