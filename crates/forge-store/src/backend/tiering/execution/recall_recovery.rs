use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    failure::{StoreError, StoreErrorKind},
};

pub(crate) fn recover_recall_state<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Result<(), StoreError> {
    for record in backend.state().tier_recall_records.values() {
        if record.completion_state == crate::backend::records::TierRecallCompletionState::InFlight
            && record.artifact_key.is_empty()
        {
            return Err(StoreError::new(
                StoreErrorKind::TierRecallExecutionViolation,
                "in-flight tier recall records must keep their artifact identity across restart",
            ));
        }
    }
    Ok(())
}
