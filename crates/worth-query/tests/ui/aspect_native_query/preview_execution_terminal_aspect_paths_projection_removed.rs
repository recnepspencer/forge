use worth_query::facade::runtime::WorthQueryPreviewExecutionEvidence;

fn assert_no_terminal_path_projection(evidence: &WorthQueryPreviewExecutionEvidence) {
    let _ = evidence.terminal_aspect_paths_projection();
}

fn main() {}
