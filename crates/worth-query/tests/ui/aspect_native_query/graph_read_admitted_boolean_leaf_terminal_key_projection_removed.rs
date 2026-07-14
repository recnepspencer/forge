use worth_query::facade::runtime::WorthQueryAdmittedBooleanPredicateLeaf;

fn assert_terminal_key_projection_removed(row: &WorthQueryAdmittedBooleanPredicateLeaf) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn main() {}
