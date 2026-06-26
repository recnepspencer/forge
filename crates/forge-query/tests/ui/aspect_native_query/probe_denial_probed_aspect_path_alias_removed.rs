use forge_query::facade::ForgeQueryExistingTruthProbeDenial;

fn main() {
    let denial: ForgeQueryExistingTruthProbeDenial = unreachable!();
    let _ = denial.probed_aspect_path();
}
