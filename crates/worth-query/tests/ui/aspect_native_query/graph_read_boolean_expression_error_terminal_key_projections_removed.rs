use worth_query::facade::WorthQueryBooleanExpressionAdmissionError;

fn main() {
    let error: WorthQueryBooleanExpressionAdmissionError = unreachable!();
    let _ = error.terminal_aspect_projection();
    let _ = error.terminal_field_projection();
}
