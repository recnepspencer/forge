use worth_store_physical_certification::{PhysicalBoundarySeam, PhysicalBoundaryYieldpoint};

fn main() {
    let _WORTHd = PhysicalBoundaryYieldpoint {
        name: "memory-pressure-boundary".to_owned(),
        seam: PhysicalBoundarySeam::MemoryPressure,
    };
}
