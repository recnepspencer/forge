use crate::{
    backend::records::StoreState,
    failure::{StoreError, StoreErrorKind},
    retention::{RetentionMaintenanceVerification, RetentionTargetStateVerification},
};

pub(super) fn maintenance_verification(
    state: &StoreState,
    operation_label: &str,
    target_state: Option<RetentionTargetStateVerification>,
) -> Result<RetentionMaintenanceVerification, StoreError> {
    let truth_export = state.authoritative_export_bundle().into_canonicalized();
    let truth_digest = truth_export.canonical_json();
    let restored_state = StoreState::from_authoritative_export_bundle(truth_export)?;
    let restore_digest = restored_state
        .authoritative_export_bundle()
        .into_canonicalized()
        .canonical_json();
    if let Some(target_state) = &target_state {
        if !target_state.matches_expectation() {
            return Err(StoreError::new(
                StoreErrorKind::ReclaimEligibilityViolation,
                format!(
                    "{operation_label} verification expected `{}` target `{}` presence to be {} but observed {}",
                    target_state.family_label(),
                    target_state.target_id(),
                    target_state.expected_present(),
                    target_state.observed_present()
                ),
            ));
        }
    }
    if truth_digest != restore_digest {
        return Err(StoreError::new(
            StoreErrorKind::CompactionCutoverViolation,
            format!(
                "{operation_label} verification detected restore-parity drift after maintenance"
            ),
        ));
    }
    Ok(RetentionMaintenanceVerification::new(
        operation_label,
        truth_digest.clone(),
        restore_digest.clone(),
        truth_digest == restore_digest,
        target_state,
    ))
}
