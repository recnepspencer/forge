use worth_store::{CompatibilityMaintenanceAdmissionWitness, CompatibilityMaintenanceLaneAdmission};

fn main() {
    let _ = CompatibilityMaintenanceLaneAdmission::new(witness());
}

fn witness() -> CompatibilityMaintenanceAdmissionWitness {
    panic!("compile-fail fixture")
}
