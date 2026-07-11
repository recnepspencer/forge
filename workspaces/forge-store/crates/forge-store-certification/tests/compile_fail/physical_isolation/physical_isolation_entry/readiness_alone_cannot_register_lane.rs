use forge_store_physical_certification::{
    register_physical_isolation_certification_lane, PhysicalIsolationHarnessReadiness,
};
use forge_store_physical_isolation::PhysicalIsolationEntryAdmission;

fn register_without_receipt(
    entry: &PhysicalIsolationEntryAdmission,
    readiness: PhysicalIsolationHarnessReadiness,
) {
    let _ = register_physical_isolation_certification_lane(entry, readiness);
}

fn main() {}
