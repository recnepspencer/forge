use forge_query::facade::ForgeQueryVerifiedAssumptionSet;

fn assert_no_neutral_asserted_path_alias(assumptions: &ForgeQueryVerifiedAssumptionSet) {
    let _ = assumptions.asserted_aspect_paths();
}

fn main() {}
