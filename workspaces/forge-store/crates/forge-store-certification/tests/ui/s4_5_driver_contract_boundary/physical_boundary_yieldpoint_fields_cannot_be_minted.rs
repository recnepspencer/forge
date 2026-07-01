use forge_store_physical_certification::{PhysicalBoundarySeam, PhysicalBoundaryYieldpoint};

fn main() {
    let _forged = PhysicalBoundaryYieldpoint {
        name: "memory-pressure-boundary".to_owned(),
        seam: PhysicalBoundarySeam::MemoryPressure,
    };
}
