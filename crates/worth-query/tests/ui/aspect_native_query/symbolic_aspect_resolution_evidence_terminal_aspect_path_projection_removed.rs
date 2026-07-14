use worth_query::facade::runtime::WorthQuerySymbolicAspectResolutionEvidence;

fn assert_terminal_projection_removed(evidence: &WorthQuerySymbolicAspectResolutionEvidence) {
    let _ = evidence.terminal_aspect_path_projection();
}

fn main() {}
