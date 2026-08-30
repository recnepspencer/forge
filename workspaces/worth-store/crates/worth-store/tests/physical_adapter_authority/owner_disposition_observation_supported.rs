use worth_store::physical_runtime::{
    PhysicalArtifactDisposition, PhysicalArtifactRoleDisposition,
};

fn inspect_owner_observation(disposition: PhysicalArtifactDisposition) {
    let _ = disposition.validator_outcome();
    match disposition.owner_role() {
        Some(PhysicalArtifactRoleDisposition::IntactAuthority(observation)) => {
            let _ = observation.scope();
        }
        Some(PhysicalArtifactRoleDisposition::DamagedAuthority(observation)) => {
            let _ = observation.localization();
        }
        Some(PhysicalArtifactRoleDisposition::RebuildableDerived(observation)) => {
            let _ = observation.damaged_derived_scope();
            let _ = observation.intact_authoritative_basis_scope();
        }
        None => {}
    }
}

fn main() {
    let _ = inspect_owner_observation;
}
