use worth_query::facade::runtime::WorthQueryBooleanPredicateSelectivityRow;

fn assert_terminal_key_projection_removed(row: &WorthQueryBooleanPredicateSelectivityRow) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn main() {}
