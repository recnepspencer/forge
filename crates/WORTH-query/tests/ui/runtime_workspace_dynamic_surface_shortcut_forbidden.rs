use worth_query::facade::WorthQueryWorkspace;

fn cannot_build_public_surfaces_through_dynamic_shortcuts(mut workspace: WorthQueryWorkspace) {
    let _ = workspace.surface("editor.canvas", "Task");
}

fn main() {}
