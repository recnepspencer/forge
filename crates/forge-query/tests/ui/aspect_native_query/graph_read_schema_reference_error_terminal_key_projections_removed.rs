use forge_query::facade::ForgeQueryGraphReadSchemaReferenceAdmissionError;

fn main() {
    let error: ForgeQueryGraphReadSchemaReferenceAdmissionError = unreachable!();
    let _ = error.terminal_aspect_projection();
    let _ = error.terminal_field_projection();
}
