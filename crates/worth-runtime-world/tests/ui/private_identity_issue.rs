use worth_runtime_world::facade::{CompositeBasisIdentity, RuntimeWorldOwnerIdentity};

fn placeholder<T>() -> T {
    loop {}
}

fn issue(owner: RuntimeWorldOwnerIdentity) -> CompositeBasisIdentity {
    CompositeBasisIdentity::issued(
        owner,
        placeholder(),
        placeholder(),
        placeholder(),
    )
}

fn main() {}
