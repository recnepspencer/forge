use worth_query::facade::WorthQueryPreviewExecutionEvidence;

fn assert_no_neutral_path_alias(evidence: &WorthQueryPreviewExecutionEvidence) {
    let _ = evidence.aspect_paths();
}

fn main() {}
