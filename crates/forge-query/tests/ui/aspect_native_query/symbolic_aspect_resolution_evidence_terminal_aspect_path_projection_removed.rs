use forge_query::facade::ForgeQuerySymbolicAspectResolutionEvidence;

fn assert_terminal_projection_removed(evidence: &ForgeQuerySymbolicAspectResolutionEvidence) {
    let _ = evidence.terminal_aspect_path_projection();
}

fn main() {}
