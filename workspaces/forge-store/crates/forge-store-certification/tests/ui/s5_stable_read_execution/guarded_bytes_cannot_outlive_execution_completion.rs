use forge_store_physical_isolation::{
    PhysicalByteGuard, StablePhysicalReadExecution,
};

fn misuse<'a>(mut execution: StablePhysicalReadExecution, guard: PhysicalByteGuard<'a>) {
    let guarded = execution.read_guarded_bytes(&guard).unwrap();
    let _receipt = execution.complete();
    let _bytes = guarded.physical_bytes();
}

fn main() {}
