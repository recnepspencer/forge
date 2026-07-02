use forge_store_physical_certification::register_s5_physical_isolation_certification_lane;
use forge_store_physical_isolation::{
    PhysicalIsolationEntryAdmission, PhysicalIsolationEntryEvidence,
};

fn register_with_entry_evidence(
    entry: &PhysicalIsolationEntryAdmission,
    evidence: PhysicalIsolationEntryEvidence,
) {
    let _ = register_s5_physical_isolation_certification_lane(entry, evidence);
}

fn main() {}
