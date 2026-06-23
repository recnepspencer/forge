use crate::capability::CapabilitySnapshot;
use crate::runtime::{WorthUiCapabilityChangedFacts, WorthUiRuntimeFactSet};

use super::WorthUiCapabilityReloadFamilyRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedCapabilityReloadBatch {
    candidate_snapshot: CapabilitySnapshot,
    family_rows: Vec<WorthUiCapabilityReloadFamilyRow>,
    changed_facts: WorthUiCapabilityChangedFacts,
}

impl WorthUiAdmittedCapabilityReloadBatch {
    pub(crate) fn new(
        candidate_snapshot: CapabilitySnapshot,
        family_rows: Vec<WorthUiCapabilityReloadFamilyRow>,
        changed_facts: WorthUiRuntimeFactSet,
        active_snapshot_digest_before: u64,
        active_snapshot_digest_after: u64,
    ) -> Self {
        Self {
            candidate_snapshot,
            family_rows,
            changed_facts: WorthUiCapabilityChangedFacts::from_admitted_capability_reload(
                changed_facts,
                active_snapshot_digest_before,
                active_snapshot_digest_after,
            ),
        }
    }

    pub(crate) fn into_candidate_snapshot(self) -> CapabilitySnapshot {
        self.candidate_snapshot
    }

    pub fn candidate_snapshot(&self) -> &CapabilitySnapshot {
        &self.candidate_snapshot
    }

    pub fn family_rows(&self) -> &[WorthUiCapabilityReloadFamilyRow] {
        &self.family_rows
    }

    pub fn changed_facts(&self) -> &WorthUiCapabilityChangedFacts {
        &self.changed_facts
    }
}
