use forge_query::facade::ForgeQueryWorkspace;

fn cannot_escape_submission_runtime(mut workspace: ForgeQueryWorkspace) {
    let mut lane = workspace.submissions().unwrap();
    let _ = lane.runtime;
}

fn main() {}
