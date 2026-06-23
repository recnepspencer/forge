use forge_query::facade::ForgeQueryPreviewExecutionEvidence;

fn assert_no_neutral_path_alias(evidence: &ForgeQueryPreviewExecutionEvidence) {
    let _ = evidence.aspect_paths();
}

fn main() {}
