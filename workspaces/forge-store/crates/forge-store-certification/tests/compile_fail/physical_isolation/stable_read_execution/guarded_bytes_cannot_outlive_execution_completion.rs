use forge_store_physical_isolation::{
    LogicalDecodeSecurityScopeEntry, PhysicalByteGuard, StablePhysicalReadExecution,
};

fn misuse<'a>(
    mut execution: StablePhysicalReadExecution,
    guard: PhysicalByteGuard<'a>,
    entry: LogicalDecodeSecurityScopeEntry,
) {
    let guarded = execution
        .read_guarded_bytes_with_security_scope(&guard, entry)
        .unwrap();
    let _receipt = execution.complete();
    let _bytes = guarded.physical_bytes();
}

fn main() {}
