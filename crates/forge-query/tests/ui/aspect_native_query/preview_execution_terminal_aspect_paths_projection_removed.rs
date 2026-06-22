use forge_query::facade::ForgeQueryPreviewExecutionEvidence;

fn assert_no_terminal_path_projection(evidence: &ForgeQueryPreviewExecutionEvidence) {
    let _ = evidence.terminal_aspect_paths_projection();
}

fn main() {}
