use forge_query::facade::{
    ForgeQueryOperationInput, ForgeQueryOperationOutput, ForgeQueryProgramValue,
};

fn main() {
    let value = ForgeQueryProgramValue::string("native");
    let _ = value.terminal_json_projection();

    let input = ForgeQueryOperationInput::new("input", ForgeQueryProgramValue::string("native"));
    let _ = input.terminal_json_value_projection();

    let output = ForgeQueryOperationOutput::new("output", ForgeQueryProgramValue::string("native"));
    let _ = output.terminal_json_value_projection();
}
