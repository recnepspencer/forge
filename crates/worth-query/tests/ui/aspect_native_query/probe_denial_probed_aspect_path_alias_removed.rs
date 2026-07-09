use worth_query::facade::WorthQueryExistingTruthProbeDenial;

fn main() {
    let denial: WorthQueryExistingTruthProbeDenial = unreachable!();
    let _ = denial.probed_aspect_path();
}
