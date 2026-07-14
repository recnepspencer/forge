use worth_query::facade::runtime::WorthQueryExistingTruthProbeDenial;

fn main() {
    let denial: WorthQueryExistingTruthProbeDenial = unreachable!();
    let _ = denial.probed_aspect_path();
}
