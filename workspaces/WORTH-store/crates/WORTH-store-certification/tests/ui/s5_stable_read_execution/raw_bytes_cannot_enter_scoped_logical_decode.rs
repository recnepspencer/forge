use worth_store_physical_isolation::{
    LogicalDecodeSecurityScopeEntry, StablePhysicalReadExecution,
};

fn misuse(mut execution: StablePhysicalReadExecution, entry: LogicalDecodeSecurityScopeEntry) {
    let bytes: &[u8] = b"unscoped";
    let _ = execution.read_guarded_bytes_with_security_scope(bytes, entry);
}

fn main() {}
