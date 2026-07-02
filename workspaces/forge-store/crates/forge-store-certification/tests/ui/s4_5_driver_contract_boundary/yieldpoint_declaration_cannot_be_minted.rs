use forge_store_physical_certification::{PhysicalBoundaryYieldpoint, YieldpointDeclaration};

fn main() {
    let _forged = YieldpointDeclaration {
        yieldpoint: PhysicalBoundaryYieldpoint::memory_pressure_boundary(),
    };
}
