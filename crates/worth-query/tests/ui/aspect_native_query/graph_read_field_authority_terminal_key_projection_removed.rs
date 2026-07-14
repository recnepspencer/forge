use worth_query::facade::runtime::{WorthQueryGraphReadOrderingFieldAuthority, WorthQueryGraphReadPredicateFieldAuthority};

fn assert_predicate_terminal_key_projection_removed(
    row: &WorthQueryGraphReadPredicateFieldAuthority,
) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn assert_ordering_terminal_key_projection_removed(row: &WorthQueryGraphReadOrderingFieldAuthority) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn main() {}
