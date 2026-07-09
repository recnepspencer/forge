use worth_store_physical_isolation::{
    CurrentPhysicalRoot, PhysicalByteGuard, StablePhysicalReadExecution,
};

fn misuse<'a>(
    mut execution: StablePhysicalReadExecution,
    guard: PhysicalByteGuard<'a>,
    root: CurrentPhysicalRoot,
) {
    let _ = execution.read_guarded_bytes_with_security_scope(&guard, root);
}

fn main() {}
