use worth_query::facade::runtime::WorthQueryExistingTruthAssertionDenial;

fn main() {
    let denial: WorthQueryExistingTruthAssertionDenial = worth_query_denial();
    let _ = denial.expected_native_value_digest();
    let _ = denial.found_native_value_digest();
}

fn worth_query_denial() -> WorthQueryExistingTruthAssertionDenial {
    unreachable!()
}
