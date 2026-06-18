use forge_query::facade::runtime::{ForgeQueryExistingRelationTarget, ForgeQueryWorkspace};

fn forbidden(workspace: ForgeQueryWorkspace, target: ForgeQueryExistingRelationTarget) {
    let _ = workspace.bind_existing_relation(target);
}

fn main() {}
