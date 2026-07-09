use worth_query::facade::WorthQueryExistingTruthAssertionDenial;

fn main() {
    let denial: WorthQueryExistingTruthAssertionDenial = unreachable!();
    let _ = denial.asserted_aspect_path();
}
