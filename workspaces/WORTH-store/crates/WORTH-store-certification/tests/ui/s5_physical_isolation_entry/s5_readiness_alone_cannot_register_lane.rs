use worth_store_physical_certification::{
    register_s5_physical_isolation_certification_lane, S5SimulationHarnessReadiness,
};
use worth_store_physical_isolation::PhysicalIsolationEntryAdmission;

fn register_without_receipt(
    entry: &PhysicalIsolationEntryAdmission,
    readiness: S5SimulationHarnessReadiness,
) {
    let _ = register_s5_physical_isolation_certification_lane(entry, readiness);
}

fn main() {}
