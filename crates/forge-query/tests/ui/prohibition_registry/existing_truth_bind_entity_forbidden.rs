use forge_query::facade::runtime::{ForgeQueryExistingEntityTarget, ForgeQueryWorkspace};

fn forbidden(workspace: ForgeQueryWorkspace, target: ForgeQueryExistingEntityTarget) {
    let _ = workspace.bind_existing_entity(target);
}

fn main() {}
