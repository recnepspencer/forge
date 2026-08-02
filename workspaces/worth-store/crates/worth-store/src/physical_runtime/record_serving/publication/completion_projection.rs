use worth_store_physical_format::PersistedRecordIdentity;

use super::append_observation::{PublicationObservation, RecordAppendObservation};

/// Per-member caller result material preserved independently of root assembly.
///
/// This is derived output, not root, WAL, data-settlement, or acknowledgment
/// authority. The matching proof-bearing member remains its sole authority.
pub(in crate::physical_runtime) struct PreparedRecordCompletionProjection {
    records: Box<[PersistedRecordIdentity]>,
    observation: RecordAppendObservation,
}

impl PreparedRecordCompletionProjection {
    pub(in crate::physical_runtime::record_serving) fn new(
        records: &[PersistedRecordIdentity],
        observation: PublicationObservation,
    ) -> Self {
        Self {
            records: records.to_vec().into_boxed_slice(),
            observation: RecordAppendObservation::from_publication(observation),
        }
    }

    pub(in crate::physical_runtime) fn records(&self) -> &[PersistedRecordIdentity] {
        &self.records
    }

    pub(in crate::physical_runtime) const fn observation(&self) -> RecordAppendObservation {
        self.observation
    }
}
