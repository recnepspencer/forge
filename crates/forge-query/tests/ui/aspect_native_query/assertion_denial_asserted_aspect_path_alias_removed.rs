use forge_query::facade::ForgeQueryExistingTruthAssertionDenial;

fn main() {
    let denial: ForgeQueryExistingTruthAssertionDenial = unreachable!();
    let _ = denial.asserted_aspect_path();
}
