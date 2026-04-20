use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
};

impl StoreState {
    pub fn verify_tiering_record_family(&self) -> Result<(), StoreError> {
        for record in self.tier_residency_records.values() {
            if record.artifact_key.is_empty()
                || record.canonical_replica_locator.is_empty()
                || record.verification_label.is_empty()
            {
                return Err(StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    "tier residency records must declare non-empty artifact identity, locator, and verification label",
                ));
            }
        }

        for record in self.tier_transfer_records.values() {
            if record.artifact_key.is_empty() || record.source_replica_locator.is_empty() {
                return Err(StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    "tier transfer records must declare non-empty artifact identity and source locator",
                ));
            }
            if record.cutover_completed && record.transferred_replica_locator.is_none() {
                return Err(StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    format!(
                        "tier transfer `{}` cannot complete cutover without a transferred replica locator",
                        record.artifact_key
                    ),
                ));
            }
            if record.cutover_completed && record.verification_label.is_none() {
                return Err(StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    format!(
                        "tier transfer `{}` cannot complete cutover without a verification label",
                        record.artifact_key
                    ),
                ));
            }
        }

        Ok(())
    }
}
