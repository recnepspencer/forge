use forge_store_physical_isolation::StablePhysicalReadExecution;

fn main() {
    let mut execution: StablePhysicalReadExecution = todo!();
    let bytes: &[u8] = b"unprotected";
    let _read = execution.read_guarded_bytes(bytes);
}
