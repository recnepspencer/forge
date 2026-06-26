use forge_query::facade::ForgeQueryExistingTruthAssertionDenial;

fn main() {
    let denial: ForgeQueryExistingTruthAssertionDenial = forge_query_denial();
    let _ = denial.expected_native_value_digest();
    let _ = denial.found_native_value_digest();
}

fn forge_query_denial() -> ForgeQueryExistingTruthAssertionDenial {
    unreachable!()
}
