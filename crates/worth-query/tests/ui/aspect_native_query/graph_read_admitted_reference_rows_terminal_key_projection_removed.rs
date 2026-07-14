use worth_query::facade::runtime::{WorthQueryAdmittedGraphReadOrderingField, WorthQueryAdmittedGraphReadPredicateField, WorthQueryAdmittedGraphReadProjectionField};

fn assert_projection_terminal_key_projection_removed(
    row: &WorthQueryAdmittedGraphReadProjectionField,
) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn assert_predicate_terminal_key_projection_removed(row: &WorthQueryAdmittedGraphReadPredicateField) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn assert_ordering_terminal_key_projection_removed(row: &WorthQueryAdmittedGraphReadOrderingField) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn main() {}
