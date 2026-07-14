use worth_query::facade::runtime::WorthQueryWorkspace;

fn cannot_escape_submission_runtime(mut workspace: WorthQueryWorkspace) {
    let mut lane = workspace.submissions().unwrap();
    let _ = lane.runtime;
}

fn main() {}
