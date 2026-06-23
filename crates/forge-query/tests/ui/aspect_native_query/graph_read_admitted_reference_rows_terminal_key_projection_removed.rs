use forge_query::facade::runtime::{
    ForgeQueryAdmittedGraphReadOrderingField, ForgeQueryAdmittedGraphReadPredicateField,
    ForgeQueryAdmittedGraphReadProjectionField,
};

fn assert_projection_terminal_key_projection_removed(
    row: &ForgeQueryAdmittedGraphReadProjectionField,
) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn assert_predicate_terminal_key_projection_removed(row: &ForgeQueryAdmittedGraphReadPredicateField) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn assert_ordering_terminal_key_projection_removed(row: &ForgeQueryAdmittedGraphReadOrderingField) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn main() {}
