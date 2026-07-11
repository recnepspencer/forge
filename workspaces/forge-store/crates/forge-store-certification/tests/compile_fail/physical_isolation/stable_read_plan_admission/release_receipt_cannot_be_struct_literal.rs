use forge_store_physical_isolation::{
    PhysicalReadPlanReleaseReceipt, PhysicalReadProtectedFootprintBasis,
};

fn main() {
    let footprint_basis: PhysicalReadProtectedFootprintBasis = todo!();
    let _receipt = PhysicalReadPlanReleaseReceipt {
        footprint_basis,
    };
}
