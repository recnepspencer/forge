use forge_query::facade::runtime::ForgeQueryRetainedScalarAlignmentFact;

fn assert_terminal_field_key_projections_removed(row: &ForgeQueryRetainedScalarAlignmentFact) {
    let _ = row.terminal_left_field_key_projection();
    let _ = row.terminal_right_field_key_projection();
}

fn main() {}
