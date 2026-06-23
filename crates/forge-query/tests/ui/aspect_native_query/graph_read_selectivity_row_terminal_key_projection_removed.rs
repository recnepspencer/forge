use forge_query::facade::runtime::ForgeQueryBooleanPredicateSelectivityRow;

fn assert_terminal_key_projection_removed(row: &ForgeQueryBooleanPredicateSelectivityRow) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn main() {}
