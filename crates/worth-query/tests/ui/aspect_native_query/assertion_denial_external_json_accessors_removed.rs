use worth_query::facade::WorthQueryExistingTruthAssertionDenial;

fn main() {
    let denial: WorthQueryExistingTruthAssertionDenial = unreachable!();
    let _ = denial.expected_external_value_json();
    let _ = denial.found_external_value_json();
}
