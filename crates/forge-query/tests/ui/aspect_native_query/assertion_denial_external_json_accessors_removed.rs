use forge_query::facade::ForgeQueryExistingTruthAssertionDenial;

fn main() {
    let denial: ForgeQueryExistingTruthAssertionDenial = unreachable!();
    let _ = denial.expected_external_value_json();
    let _ = denial.found_external_value_json();
}
