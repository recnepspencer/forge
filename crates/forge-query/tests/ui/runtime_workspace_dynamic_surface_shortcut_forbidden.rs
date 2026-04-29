use forge_query::facade::ForgeQueryWorkspace;

fn cannot_build_public_surfaces_through_dynamic_shortcuts(mut workspace: ForgeQueryWorkspace) {
    let _ = workspace.surface("editor.canvas", "Task");
}

fn main() {}
