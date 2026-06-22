use forge_query::facade::ForgeQueryExistingTruthProbeRequest;

fn assert_no_neutral_path_alias(request: &ForgeQueryExistingTruthProbeRequest) {
    let _ = request.aspect_paths();
}

fn main() {}
