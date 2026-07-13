use worth_query::facade::runtime::WorthQueryExistingTruthAssertionDenial;

fn main() {
    let denial: WorthQueryExistingTruthAssertionDenial = unreachable!();
    let _ = denial.asserted_aspect_path();
}
