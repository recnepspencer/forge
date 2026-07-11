use forge_store_physical_certification::{PhysicalBoundaryYieldpoint, YieldpointPauseReceipt};

fn main() {
    let _receipt = YieldpointPauseReceipt {
        yieldpoint: PhysicalBoundaryYieldpoint::root_publication_before_observe(),
    };
}
