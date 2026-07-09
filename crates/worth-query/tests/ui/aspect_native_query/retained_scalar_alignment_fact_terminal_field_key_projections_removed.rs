use worth_query::facade::runtime::WorthQueryRetainedScalarAlignmentFact;

fn assert_terminal_field_key_projections_removed(row: &WorthQueryRetainedScalarAlignmentFact) {
    let _ = row.terminal_left_field_key_projection();
    let _ = row.terminal_right_field_key_projection();
}

fn main() {}
