use worth_query::facade::runtime::{WorthQueryExistingEntityTarget, WorthQueryWorkspace};

fn forbidden(workspace: WorthQueryWorkspace, target: WorthQueryExistingEntityTarget) {
    let _ = workspace.bind_existing_entity(target);
}

fn main() {}
