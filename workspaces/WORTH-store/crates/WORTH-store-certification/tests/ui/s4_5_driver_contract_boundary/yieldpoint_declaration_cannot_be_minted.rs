use worth_store_physical_certification::{PhysicalBoundaryYieldpoint, YieldpointDeclaration};

fn main() {
    let _WORTHd = YieldpointDeclaration {
        yieldpoint: PhysicalBoundaryYieldpoint::memory_pressure_boundary(),
    };
}
