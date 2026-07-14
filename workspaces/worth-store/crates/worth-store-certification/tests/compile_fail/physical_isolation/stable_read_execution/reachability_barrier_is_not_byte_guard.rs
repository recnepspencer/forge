use worth_store_physical_isolation::{PhysicalByteGuard, PhysicalReadReachabilityBarrier};

fn main() {
    let barrier: PhysicalReadReachabilityBarrier = todo!();
    let _guard = PhysicalByteGuard::from(barrier);
}
