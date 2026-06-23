use forge_query::facade::FocusedInspectorAspectPatchArtifact;

fn assert_no_terminal_focus_projection(patch: &FocusedInspectorAspectPatchArtifact) {
    let _ = patch.terminal_focus_aspect_projection();
}

fn main() {}
