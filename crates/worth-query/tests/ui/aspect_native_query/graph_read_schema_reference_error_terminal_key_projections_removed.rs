use worth_query::facade::runtime::WorthQueryGraphReadSchemaReferenceAdmissionError;

fn main() {
    let error: WorthQueryGraphReadSchemaReferenceAdmissionError = unreachable!();
    let _ = error.terminal_aspect_projection();
    let _ = error.terminal_field_projection();
}
