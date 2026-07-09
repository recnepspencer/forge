use worth_query::facade::runtime::WorthQueryRetainedScalarFieldFact;

fn assert_terminal_field_key_projection_removed(row: &WorthQueryRetainedScalarFieldFact) {
    let _ = row.terminal_field_key_projection();
}

fn main() {}
