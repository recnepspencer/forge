mod artifact_inventory;
mod observation;

use std::path::Path;

use worth_store_physical_format::PhysicalRecordFormatDeclaration;

pub use observation::{
    OfflineHostileArtifactObservation, OfflineHostileCurrentRecordTruth,
    OfflineHostilePhysicalTruthBudget, OfflineHostilePhysicalTruthBudgetDenial,
    OfflineHostilePhysicalTruthDenial, OfflineHostilePhysicalTruthObservation,
};

pub fn observe_hostile_physical_truth(
    store_root: &Path,
    expected_format: PhysicalRecordFormatDeclaration,
    budget: OfflineHostilePhysicalTruthBudget,
) -> Result<OfflineHostilePhysicalTruthObservation, OfflineHostilePhysicalTruthDenial> {
    let artifacts = artifact_inventory::inventory(store_root, budget)?;
    let current = super::walk_current_durable_record_manifest(store_root, expected_format)
        .map(OfflineHostileCurrentRecordTruth::from_walk);
    Ok(OfflineHostilePhysicalTruthObservation::new(
        artifacts, current,
    ))
}

#[cfg(test)]
mod tests;
