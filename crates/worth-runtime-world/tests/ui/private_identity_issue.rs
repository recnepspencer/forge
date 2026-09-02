use worth_runtime_world::facade::{CompositeBasisIdentity, RuntimeWorldOwnerIdentity};

fn issue(owner: RuntimeWorldOwnerIdentity) -> CompositeBasisIdentity {
    CompositeBasisIdentity::issued(owner, 0)
}

fn main() {}
