use forge_store_physical_isolation::{PhysicalIsolationRootEpochBasis, RootEpoch};

fn main() {
    let _ = PhysicalIsolationRootEpochBasis {
        epoch: RootEpoch(1),
    };
}
