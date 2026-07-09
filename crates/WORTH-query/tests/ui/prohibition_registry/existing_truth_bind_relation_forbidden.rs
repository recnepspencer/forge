use worth_query::facade::runtime::{WorthQueryExistingRelationTarget, WorthQueryWorkspace};

fn forbidden(workspace: WorthQueryWorkspace, target: WorthQueryExistingRelationTarget) {
    let _ = workspace.bind_existing_relation(target);
}

fn main() {}
