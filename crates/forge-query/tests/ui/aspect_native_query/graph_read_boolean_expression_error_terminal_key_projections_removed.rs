use forge_query::facade::ForgeQueryBooleanExpressionAdmissionError;

fn main() {
    let error: ForgeQueryBooleanExpressionAdmissionError = unreachable!();
    let _ = error.terminal_aspect_projection();
    let _ = error.terminal_field_projection();
}
