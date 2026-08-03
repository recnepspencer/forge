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
    let (current, record_payloads) =
        match super::walk_current_durable_record_manifest(store_root, expected_format) {
            Ok(walk) => (
                Ok(OfflineHostileCurrentRecordTruth::from_walk(&walk)),
                walk.record_payloads().to_vec(),
            ),
            Err(denial) => (Err(denial), Vec::new()),
        };
    Ok(OfflineHostilePhysicalTruthObservation::new(
        artifacts,
        current,
        record_payloads,
    ))
}

#[cfg(test)]
mod tests;
