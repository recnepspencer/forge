use forge_query::facade::runtime::ForgeQueryRetainedScalarFieldFact;

fn assert_terminal_field_key_projection_removed(row: &ForgeQueryRetainedScalarFieldFact) {
    let _ = row.terminal_field_key_projection();
}

fn main() {}
