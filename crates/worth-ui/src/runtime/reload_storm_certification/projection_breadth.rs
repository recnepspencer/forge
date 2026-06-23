use crate::runtime::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiProjectionRebindBatchReceipt,
    WorthUiRuntimeChangeEvidenceDigest,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiProjectionRebindBatchDigest(u64);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadProjectionBreadthCertification {
    change_evidence_digest: WorthUiRuntimeChangeEvidenceDigest,
    changed_fact_count: usize,
    projection_rebind_batch_digest: WorthUiProjectionRebindBatchDigest,
    inspected_projection_count: usize,
    dependency_intersection_count: usize,
    rebuild_attempt_count: usize,
    preserved_frame_count: usize,
    denied_frame_count: usize,
    rebuilt_frame_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiReloadProjectionBreadthDenial {
    RuntimeInstanceMismatch,
    ChangeEvidenceDigestMismatch,
    EmptyChangedFactsForRebuild,
    RebuildCountDoesNotMatchDependencyIntersections,
    RebuiltFrameCountDoesNotMatchAttempts,
}

impl WorthUiProjectionRebindBatchDigest {
    pub(crate) fn from_batch(batch: &WorthUiProjectionRebindBatchReceipt) -> Self {
        let mut entries = vec![
            format!("change:{:?}", batch.change_evidence_digest()),
            format!("counters:{:?}", batch.counters()),
        ];
        for row in batch.rows() {
            entries.push(format!(
                "row:{}|{:?}|{:?}|{}|{}",
                row.projection_identity().as_str(),
                row.projection_family(),
                row.status(),
                row.previous_frame_digest(),
                row.rebound_frame_digest()
            ));
        }
        Self(super::digest::fold_texts(entries))
    }

    pub fn raw(self) -> u64 {
        self.0
    }
}

impl WorthUiReloadProjectionBreadthCertification {
    pub fn certify(
        change_evidence: &WorthUiAdmittedRuntimeChangeEvidence,
        batch: &WorthUiProjectionRebindBatchReceipt,
    ) -> Result<Self, WorthUiReloadProjectionBreadthDenial> {
        if change_evidence.runtime_instance() != batch.runtime_instance() {
            return Err(WorthUiReloadProjectionBreadthDenial::RuntimeInstanceMismatch);
        }
        if change_evidence.digest() != batch.change_evidence_digest() {
            return Err(WorthUiReloadProjectionBreadthDenial::ChangeEvidenceDigestMismatch);
        }
        let counters = batch.counters();
        let changed_fact_count = admitted_changed_fact_count(change_evidence);
        if changed_fact_count == 0 && counters.rebuild_attempt_count() > 0 {
            return Err(WorthUiReloadProjectionBreadthDenial::EmptyChangedFactsForRebuild);
        }
        if counters.rebuild_attempt_count() != counters.dependency_intersection_count() {
            return Err(
                WorthUiReloadProjectionBreadthDenial::RebuildCountDoesNotMatchDependencyIntersections,
            );
        }
        if counters.rebuilt_frame_count() != counters.rebuild_attempt_count() {
            return Err(
                WorthUiReloadProjectionBreadthDenial::RebuiltFrameCountDoesNotMatchAttempts,
            );
        }
        Ok(Self {
            change_evidence_digest: batch.change_evidence_digest(),
            changed_fact_count,
            projection_rebind_batch_digest: WorthUiProjectionRebindBatchDigest::from_batch(batch),
            inspected_projection_count: counters.inspected_projection_count(),
            dependency_intersection_count: counters.dependency_intersection_count(),
            rebuild_attempt_count: counters.rebuild_attempt_count(),
            preserved_frame_count: counters.preserved_frame_count(),
            denied_frame_count: counters.denied_frame_count(),
            rebuilt_frame_count: counters.rebuilt_frame_count(),
        })
    }

    pub fn change_evidence_digest(&self) -> WorthUiRuntimeChangeEvidenceDigest {
        self.change_evidence_digest
    }

    pub fn changed_fact_count(&self) -> usize {
        self.changed_fact_count
    }

    pub fn projection_rebind_batch_digest(&self) -> WorthUiProjectionRebindBatchDigest {
        self.projection_rebind_batch_digest
    }

    pub fn inspected_projection_count(&self) -> usize {
        self.inspected_projection_count
    }

    pub fn dependency_intersection_count(&self) -> usize {
        self.dependency_intersection_count
    }

    pub fn rebuild_attempt_count(&self) -> usize {
        self.rebuild_attempt_count
    }

    pub fn preserved_frame_count(&self) -> usize {
        self.preserved_frame_count
    }

    pub fn denied_frame_count(&self) -> usize {
        self.denied_frame_count
    }

    pub fn rebuilt_frame_count(&self) -> usize {
        self.rebuilt_frame_count
    }
}

fn admitted_changed_fact_count(change_evidence: &WorthUiAdmittedRuntimeChangeEvidence) -> usize {
    change_evidence
        .family_rows()
        .iter()
        .map(|row| row.changed_facts().len())
        .sum()
}
