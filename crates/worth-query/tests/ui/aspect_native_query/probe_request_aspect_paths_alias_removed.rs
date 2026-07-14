use worth_query::facade::runtime::WorthQueryExistingTruthProbeRequest;

fn assert_no_neutral_path_alias(request: &WorthQueryExistingTruthProbeRequest) {
    let _ = request.aspect_paths();
}

fn main() {}
