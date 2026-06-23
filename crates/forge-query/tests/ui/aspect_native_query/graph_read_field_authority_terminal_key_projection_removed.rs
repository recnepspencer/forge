use forge_query::facade::runtime::{
    ForgeQueryGraphReadOrderingFieldAuthority, ForgeQueryGraphReadPredicateFieldAuthority,
};

fn assert_predicate_terminal_key_projection_removed(
    row: &ForgeQueryGraphReadPredicateFieldAuthority,
) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn assert_ordering_terminal_key_projection_removed(row: &ForgeQueryGraphReadOrderingFieldAuthority) {
    let _ = row.terminal_aspect_key_projection();
    let _ = row.terminal_field_key_projection();
}

fn main() {}
